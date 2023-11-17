import React, {useEffect, useState} from 'react';
import {message} from 'antd';
import {allFavoriteRecords, getRecordByPage} from "@/utils/graphql";
import InfiniteScroll from 'react-infinite-scroll-component';
import RecordCard from "@/components/RecordCard";
import {RecordDocument} from "@/models/RecordDocument";
import {Base64} from "js-base64";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {EventListenerEnum} from "@/utils/consts";

const PageSize = 10;

export default function RecordList() {

    const [favoriteRecords, setFavoriteRecords] = useState<RecordDocument[]>([])
    const [hasMore, setHasMore] = useState(false);
    const [records, setRecords] = useState<RecordDocument[]>([]);
    const [endCursor, setEndCursor] = useState<string>("");

    const reloadFavoriteRecords = async () => {
        setFavoriteRecords([]);

        try {
            let allFavoriteResp = await allFavoriteRecords();
            if (!allFavoriteResp || !allFavoriteResp.documents || allFavoriteResp.documents.length <= 0) {
                message.warning("No favorite records!")
            }
            let favoriteDocuments = allFavoriteResp.documents;
            for (let document of favoriteDocuments) {
                document.fields.content = Base64.decode(document.fields.content);
                document.fields.content_preview = Base64.decode(document.fields.content_preview);
            }
            console.debug(`all ${favoriteDocuments.length} favorite items loaded!`);
            setFavoriteRecords(favoriteDocuments);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    }

    const fetchRecords = async () => {
        try {
            let res = await getRecordByPage(PageSize, endCursor, [0]);
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
            setRecords(records.concat(documents));
            setEndCursor(res.endCursor);
            message.success(`${documents.length} more items loaded!`);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    };

    const reloadRecords = async () => {
        setHasMore(false);
        setRecords([]);
        setEndCursor("");

        try {
            let res = await getRecordByPage(PageSize, "", [0]);
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
            setRecords(documents);
            setEndCursor(res.endCursor);
            console.log(`${documents.length} more items loaded!`);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    }

    const clipboardChangeListener = async (): Promise<UnlistenFn> => {
        return listen(EventListenerEnum.ChangeClipboardBackend, async (event) => {
            console.debug(`Got ChangeClipboardBackend event: ${event}`);
            await reloadRecords();
        });
    }

    const updateClipboardRecordListener = async (): Promise<UnlistenFn> => {
        return listen(EventListenerEnum.UpdateClipboardRecordBackend, async (event) => {
            console.debug(`Got UpdateClipboardRecordBackend event: ${event}`);
            reloadFavoriteRecords().then(() => {
                reloadRecords().then();
            });
        });
    }

    const deleteClipboardRecordListener = async (): Promise<UnlistenFn> => {
        return listen(EventListenerEnum.DeleteClipboardRecordBackend, async (event) => {
            console.debug(`Got DeleteClipboardRecordBackend event: ${event}`);
            reloadFavoriteRecords().then(() => {
                reloadRecords().then();
            });
        });
    }

    useEffect(() => {
        const unlistenFns: UnlistenFn[] = [];

        clipboardChangeListener()
            .then((ulf) => {
                unlistenFns.push(ulf);
            })
            .catch((err) => {
                message.error(`handle event: ${EventListenerEnum.ChangeClipboardBackend} err: ${err.message}`);
            });

        updateClipboardRecordListener()
            .then((ulf) => {
                unlistenFns.push(ulf);
            })
            .catch((err) => {
                message.error(`handle event: ${EventListenerEnum.UpdateClipboardRecordBackend} err: ${err.message}`);
            });

        deleteClipboardRecordListener()
            .then((ulf) => {
                unlistenFns.push(ulf);
            })
            .catch((err) => {
                message.error(`handle event: ${EventListenerEnum.DeleteClipboardRecordBackend} err: ${err.message}`);
            });

        reloadFavoriteRecords().then(() => {
            reloadRecords().then();
        });

        return () => {
            for (const ulf of unlistenFns) ulf();
        };
    }, []);

    return (
        <div id={"record-list-container"}>
            favorite: {favoriteRecords.length}, normal: {records.length}
            <div className={"favorite-record-list"}>
                <InfiniteScroll
                    next={() => {
                    }}
                    hasMore={false}
                    loader={<h4>Loading favorite...</h4>}
                    dataLength={favoriteRecords.length}>
                    {
                        favoriteRecords.map(item => (
                            <div key={item.meta.viewId}>
                                <RecordCard data={item}/>
                            </div>
                        ))
                    }
                </InfiniteScroll>
            </div>
            <div className={"normal-record-list"}>
                <InfiniteScroll
                    dataLength={records.length}
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
                        records.map(item => (
                            <div key={item.meta.viewId}>
                                <RecordCard data={item}/>
                            </div>
                        ))
                    }
                </InfiniteScroll>
            </div>
        </div>
    );
};
