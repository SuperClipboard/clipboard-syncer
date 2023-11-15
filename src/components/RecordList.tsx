import React, {useEffect, useState} from 'react';
import {message} from 'antd';
import {getRecordByPage} from "@/utils/graphql";
import {RecordDocument} from "@/types";
import InfiniteScroll from 'react-infinite-scroll-component';

const PageSize = 10;

export default function RecordList() {

    const [hasMore, setHasMore] = useState(false);
    const [data, setData] = useState<RecordDocument[]>([]);
    const [endCursor, setEndCursor] = useState<string>("");

    const appendData = async () => {
        try {
            let res = await getRecordByPage(PageSize, endCursor === "" ? null : endCursor, [0, 1]);
            console.log(`res: ${res}`);

            if (!res || !res.documents || res.documents.length <= 0) {
                message.warning("No records found!")
                return;
            }

            let documents = res.documents;
            for (let document of documents) {
                if (document.fields.data_type === "text") {
                    document.fields.content = atob(document.fields.content);
                    document.fields.content_preview = atob(document.fields.content_preview);
                }
            }

            setHasMore(res.hasNextPage);
            setData(data.concat(documents));
            setEndCursor(res.endCursor);
            message.success(`${documents.length} more items loaded!`);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    };

    useEffect(() => {
        appendData();
    }, []);

    return (
        <div id={"record-list-container"}>
            <InfiniteScroll
                dataLength={data.length}
                next={appendData}
                hasMore={hasMore}
                endMessage={
                    <p style={{ textAlign: 'center' }}>
                        <b>No record!</b>
                    </p>
                }
                loader={<h4>Loading...</h4>}
            >
                {
                    data.map(item => (
                        <div key={item.fields.md5} className='pin'>
                            <div>{item.fields.content_preview}: {item.fields.create_time}</div>
                        </div>
                    ))
                }
            </InfiniteScroll>
        </div>
    );
}
