import React, {useEffect, useState} from 'react';
import VirtualList from 'rc-virtual-list';
import {List, message} from 'antd';
import {getRecordByPage} from "@/utils/graphql";
import {RecordDocument} from "@/types";

const ContainerHeight = 600;
const PageSize = 10;

export default function RecordList() {

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

            setData(data.concat(documents));
            setEndCursor(() => {
                return res.endCursor;
            });
            message.success(`${documents.length} more items loaded!`);
        } catch (err) {
            message.error(`load more items failed: ${err}`);
        }
    };

    useEffect(() => {
        appendData();
    }, []);

    const onScroll = (e: React.UIEvent<HTMLElement, UIEvent>) => {
        if (e.currentTarget.scrollHeight - e.currentTarget.scrollTop === ContainerHeight) {
            appendData();
        }
    };

    return (
        <List>
            <VirtualList
                data={data}
                height={ContainerHeight}
                itemHeight={47}
                itemKey="email"
                onScroll={onScroll}
            >
                {(item: RecordDocument) => (
                    <List.Item key={item.fields.md5}>
                        <List.Item.Meta
                            title={item.fields.content_preview}
                            description={item.fields.create_time}
                        />
                        <div>{item.fields.content_preview}</div>
                    </List.Item>
                )}
            </VirtualList>
        </List>
    );
}
