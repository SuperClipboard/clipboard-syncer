import "@/css/RecoardCard.css"
import {emit} from '@tauri-apps/api/event';
import React from "react";
import {Button, Image} from "antd";
import {base64ToImage, parseImageData} from "@/utils/image";
import {EventListenerEnum} from "@/utils/consts";
import {RecordDataTypeEnum, Record} from "@/models/Record";

interface RecordCardProps {
    data: Record,
}

interface RecordCardPropsLeft {
    data: Record,
}

interface RecordCardPropsRight {
    isFavorite: number,
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

    let clickCopy = (e: React.MouseEvent<HTMLDivElement>) => {
        console.log(e);
        emit(EventListenerEnum.ChangeClipBoard, {
            message: `Tauri is ok!` + e,
        })
    }

    return (
        <div className={"RecordCardLeft"} onClick={clickCopy}>
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
