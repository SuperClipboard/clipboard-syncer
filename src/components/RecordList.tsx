import React, {useEffect, useState} from 'react';
import VirtualList from 'rc-virtual-list';
import {List, message} from 'antd';
import {invoke} from "@tauri-apps/api";
import {find_records_by_pages_command} from "@/utils/consts";
import Record from "@/models/Record";

const ContainerHeight = 600;
const PageSize = 10;

export default function RecordList() {

    const [data, setData] = useState<Record[]>([]);
    const [pageCount, setPageCount] = useState(0);

    const appendData = () => {
        invoke(find_records_by_pages_command, {
            limit: PageSize,
            offset: pageCount * PageSize
        }).then((res) => {
            let records = res as Record[];

            if (records.length <= 0) {
                message.warning("No records found!")
                return;
            }

            setData(data.concat(records));
            setPageCount((pageCount) => {
                return pageCount + 1;
            });
            message.success(`${records.length} more items loaded!`);
        }).catch(err => {
            message.error(`load more items failed: ${err}`);
        });
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
                {(item: Record) => (
                    <List.Item key={item.id}>
                        <List.Item.Meta
                            title={item.content_preview}
                            description={item.create_time}
                        />
                        <div>{item.content_preview}</div>
                    </List.Item>
                )}
            </VirtualList>
        </List>
    );
}
