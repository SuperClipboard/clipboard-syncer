import React, {useEffect, useRef, useState} from 'react';
import {message} from 'antd';
import {allFavoriteRecords, getRecordByPage} from "@/utils/graphql";
import InfiniteScroll from 'react-infinite-scroll-component';
import RecordCard from "@/components/RecordCard";
import {RecordDocument} from "@/models/RecordDocument";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {EventListenerEnum} from "@/utils/consts";
import "@/css/RecordList.css";

const PageSize = 10;

interface RecordListProps {
    searchKeyword: string;
}

export default function RecordList(props: RecordListProps) {

    const prevSearchKeywordRef = useRef<string>(props.searchKeyword);

    const [favoriteRecords, setFavoriteRecords] = useState<RecordDocument[]>([]);
    const [hasMore, setHasMore] = useState(false);
    const [records, setRecords] = useState<RecordDocument[]>([]);
    const [endCursor, setEndCursor] = useState<string>("");

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

    useEffect(() => {
        if (prevSearchKeywordRef.current !== props.searchKeyword) {
            // when searchKeyword has changed
            console.log(`searchKeyword changed from ${prevSearchKeywordRef.current} to ${props.searchKeyword}`)
            reloadRecords().then();
        }

        prevSearchKeywordRef.current = props.searchKeyword;
    }, [props.searchKeyword]);

    const reloadFavoriteRecords = async () => {
        setFavoriteRecords([]);

        try {
            const allFavoriteResp = await allFavoriteRecords();
            if (!allFavoriteResp || !allFavoriteResp.documents || allFavoriteResp.documents.length <= 0) {
                console.log("No favorite records!")
            }
            console.debug(`all ${allFavoriteResp.documents.length} favorite items loaded!`);
            setFavoriteRecords(allFavoriteResp.documents);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    }

    const fetchRecords = async () => {
        try {
            const res = await getRecordByPage(PageSize, endCursor, [0], props.searchKeyword);

            if (!res || !res.documents || res.documents.length <= 0) {
                console.log("No records found!")
                return;
            }

            setHasMore(res.hasNextPage);
            setRecords(records.concat(res.documents));
            setEndCursor(res.endCursor);
            console.log(`${res.documents.length} more items loaded!`);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    };

    const reloadRecords = async () => {
        setHasMore(false);
        setRecords([]);
        setEndCursor("");

        try {
            const res = await getRecordByPage(PageSize, "", [0], props.searchKeyword);
            if (!res || !res.documents || res.documents.length <= 0) {
                console.log("No records found!")
                return;
            }

            setHasMore(res.hasNextPage);
            setRecords(res.documents);
            setEndCursor(res.endCursor);
            console.log(`${res.documents.length} more items loaded!`);
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

    return (
        <div id={"record-list-container"}>
            favorite: {favoriteRecords.length}, current normal: {records.length}
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
                            <b className={"normal-record-list-after"}></b>
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
