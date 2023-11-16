import React, {useEffect, useState} from 'react';
import {message} from 'antd';
import {getRecordByPage} from "@/utils/graphql";
import InfiniteScroll from 'react-infinite-scroll-component';
import RecordCard from "@/components/RecordCard";
import {RecordDocument} from "@/models/RecordDocument";
import {Base64} from "js-base64";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {EventListenerEnum} from "@/utils/consts";

const PageSize = 10;

export default function RecordList() {

    const [hasMore, setHasMore] = useState(false);
    const [data, setData] = useState<RecordDocument[]>([]);
    const [endCursor, setEndCursor] = useState<string>("");

    const fetchRecords = async () => {
        try {
            let res = await getRecordByPage(PageSize, endCursor, [0, 1]);
            console.log(`res: ${res}`);

            if (!res || !res.documents || res.documents.length <= 0) {
                message.warning("No records found!")
                return;
            }

            let documents = res.documents;
            for (let document of documents) {
                document.fields.content = Base64.decode(document.fields.content);
                document.fields.content_preview = Base64.decode(document.fields.content_preview);
            }

            setHasMore(res.hasNextPage);
            setData(data.concat(documents));
            setEndCursor(res.endCursor);
            message.success(`${documents.length} more items loaded!`);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    };

    const clipboardChangeListener = async (): Promise<UnlistenFn> => {
        return listen(EventListenerEnum.ChangeClipboardBackend, async (event) => {
            console.debug(`Got ChangeClipboardBackend event: ${event}`);

            try {
                let res = await getRecordByPage(PageSize, "", [0, 1]);
                console.log(`res: ${res}`);

                if (!res || !res.documents || res.documents.length <= 0) {
                    message.warning("No records found!")
                    setHasMore(false);
                    setData([]);
                    setEndCursor("");
                    return;
                }

                let documents = res.documents;
                for (let document of documents) {
                    document.fields.content = Base64.decode(document.fields.content);
                    document.fields.content_preview = Base64.decode(document.fields.content_preview);
                }

                setHasMore(res.hasNextPage);
                setData(documents);
                setEndCursor(res.endCursor);
                console.log(`${documents.length} more items loaded!`);
            } catch (err) {
                message.error(`load more items failed: ${err}`);
            }
        });
    }

    const handleEventErr = (event: string, msg: string) => {
        message.error(`handle event: ${event} err: ${msg}`);
    }

    useEffect(() => {
        const unlistenFns: UnlistenFn[] = [];

        clipboardChangeListener()
            .then((ulf) => {
                unlistenFns.push(ulf);
            })
            .catch((err) => {
                handleEventErr(EventListenerEnum.ChangeClipboardBackend, err.message)
            });

        fetchRecords();

        return () => {
            for (const ulf of unlistenFns) ulf();
        };
    }, []);

    return (
        <div id={"record-list-container"}>
            <InfiniteScroll
                dataLength={data.length}
                next={fetchRecords}
                hasMore={hasMore}
                endMessage={
                    <p style={{textAlign: 'center'}}>
                        <b>No record!</b>
                    </p>
                }
                loader={<h4>Loading...</h4>}
            >
                {
                    data.map(item => (
                        <div key={item.fields.md5}>
                            <RecordCard data={item}/>
                        </div>
                    ))
                }
            </InfiniteScroll>
        </div>
    );
}
