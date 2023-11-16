import "@/css/RecoardCard.css"
import {emit} from '@tauri-apps/api/event';
import React from "react";
import {Button, Image} from "antd";
import {base64ToImage, parseImageData} from "@/utils/image";
import {EventListenerEnum} from "@/utils/consts";
import {Record, RecordDataTypeEnum} from "@/models/Record";
import {Base64} from "js-base64";
import {RecordDocument} from "@/models/RecordDocument";

interface RecordCardProps {
    data: RecordDocument,
}

interface RecordCardPropsLeft {
    data: RecordDocument,
}

interface RecordCardPropsRight {
    data: RecordDocument,
}

interface TapChangeClipboardFrontendMessage {
    content: string,
    data_type: string,
}

interface DeleteClipboardRecordFrontendMessage {
    view_id: string,
}

export default function RecordCard(props: RecordCardProps) {
    return (
        <div className={"RecordCard"}>
            <RecordCardLeft data={props.data}/>
            <RecordCardRight data={props.data}/>
        </div>
    )
}

function RecordCardLeft(props: RecordCardPropsLeft) {

    const isImage = props.data.fields.data_type === RecordDataTypeEnum.Image;

    let clickCopy = (_: React.MouseEvent<HTMLDivElement>, record: Record) => {
        console.debug(record);

        let content = record.content;
        if (record.data_type === RecordDataTypeEnum.Image) {
            content = btoa(content);
        } else if (record.data_type === RecordDataTypeEnum.Text) {
            content = Base64.encode(content);
        }

        let payload: TapChangeClipboardFrontendMessage = {
            content: content,
            data_type: record.data_type
        };
        emit(EventListenerEnum.TapChangeClipboardFrontend, payload);
    }

    return (
        <div className={"RecordCardLeft"} onClick={(e) => clickCopy(e, props.data.fields)}>
            {
                isImage ? (
                    <Image
                        src={URL.createObjectURL(base64ToImage((parseImageData(props.data.fields.content_preview).base64)))}
                        preview={false}/>
                ) : (
                    <p>{props.data.fields.content_preview}</p>
                )
            }
        </div>
    );
}

function RecordCardRight(props: RecordCardPropsRight) {

    let deleteRecord = (_: React.MouseEvent<HTMLElement>, doc: RecordDocument) => {
        console.debug(`delete record emit view id: ${doc.meta.viewId}`);

        let payload: DeleteClipboardRecordFrontendMessage = {
            view_id: doc.meta.viewId,
        };

        emit(EventListenerEnum.DeleteClipboardRecordFrontend, payload);
    }

    return (
        <div className={"RecordCardRight"}>
            <Button type={"primary"} ghost={true} danger={true} size={"small"} onClick={(e) => {
                deleteRecord(e, props.data)
            }}>x</Button>
        </div>
    );
}
