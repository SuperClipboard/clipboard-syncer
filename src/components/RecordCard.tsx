import "@/css/RecoardCard.css"
import {emit} from '@tauri-apps/api/event';
import React from "react";
import {Button, Image} from "antd";
import {base64ToImage, parseImageData} from "@/utils/image";
import {EventListenerEnum} from "@/utils/consts";
import {Record, RecordDataTypeEnum} from "@/models/Record";
import {Base64} from "js-base64";

interface RecordCardProps {
    data: Record,
}

interface RecordCardPropsLeft {
    data: Record,
}

interface RecordCardPropsRight {
    isFavorite: number,
}

interface TapChangeClipboardFrontendMessage {
    content: string,
    data_type: string,
}

export default function RecordCard(props: RecordCardProps) {
    return (
        <div className={"RecordCard"}>
            <RecordCardLeft data={props.data}/>
            <RecordCardRight isFavorite={props.data?.is_favorite || 0}/>
        </div>
    )
}

function RecordCardLeft(props: RecordCardPropsLeft) {

    const isImage = props.data.data_type === RecordDataTypeEnum.Image;

    let clickCopy = (e: React.MouseEvent<HTMLDivElement>, record: Record) => {
        console.debug(e, record);

        let content = record.content;
        if (record.data_type === RecordDataTypeEnum.Image) {
            content = btoa(content);
        } else if (record.data_type === RecordDataTypeEnum.Text) {
            content = Base64.encode(content);
        }

        let payload : TapChangeClipboardFrontendMessage = {
            content: content,
            data_type: record.data_type
        };
        emit(EventListenerEnum.TapChangeClipboardFrontend, payload);
    }

    return (
        <div className={"RecordCardLeft"} onClick={(e) => clickCopy(e, props.data)}>
            {
                isImage ? (
                    <Image
                        src={URL.createObjectURL(base64ToImage((parseImageData(props.data.content_preview).base64)))}
                        preview={false}/>
                ) : (
                    <p>{props.data.content_preview}</p>
                )
            }
        </div>
    );
}

function RecordCardRight(props: RecordCardPropsRight) {

    let deleteRecord = (e: React.MouseEvent<HTMLDivElement>) => {
        console.log('mock delete record', e);
    }

    return (
        <div className={"RecordCardRight"}>
            <Button type={"primary"} ghost={true} danger={true} size={"small"} onClick={deleteRecord}>x</Button>
        </div>
    );
}
