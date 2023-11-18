import "@/css/RecoardCard.css"
import React, {useState} from "react";
import {Button, Image, message} from "antd";
import {base64ToImage, parseImageData} from "@/utils/image";
import {CommandEnum} from "@/utils/consts";
import {Record, RecordDataTypeEnum} from "@/models/Record";
import {RecordDocument} from "@/models/RecordDocument";
import {invoke} from "@tauri-apps/api";
import {Tooltip} from 'react-tooltip';

interface RecordCardProps {
    data: RecordDocument,
}

interface RecordCardPropsLeft {
    data: RecordDocument,
}

interface RecordCardPropsRight {
    data: RecordDocument,
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

        invoke(CommandEnum.TapChangeClipboardCommand, {
            content: record.content,
            dataType: record.data_type
        }).catch(e => {
            message.error(`change clipboard record failed: ${e}`);
        });
    }

    return (
        <div className={"record-card-left"} onClick={(e) => clickCopy(e, props.data.fields)}>
            {
                isImage ? (
                    <Image
                        src={URL.createObjectURL(base64ToImage((parseImageData(props.data.fields.content_preview).base64)))}
                        preview={false}/>
                ) : (
                    <div className={"record-card-text-container"}>
                        <p
                            data-tooltip-id="record-card-text"
                            data-tooltip-content={props.data.fields.content}
                            data-tooltip-place={"bottom"}
                        >{props.data.fields.content_preview}</p>
                        <Tooltip id={"record-card-text"} className={"record-card-text-tooltip"}/>
                    </div>
                )
            }
        </div>
    );
}

function RecordCardRight(props: RecordCardPropsRight) {

    const [isFavorite, setIsFavorite] = useState(props.data.fields.is_favorite);

    let deleteRecord = (_: React.MouseEvent<HTMLElement>, doc: RecordDocument) => {
        // Use documentId to delete the entire document!
        console.debug(`delete record emit view id: ${doc.meta.documentId}`);

        invoke(CommandEnum.DeleteRecordCommand, {
            viewId: doc.meta.documentId
        }).catch(e => {
            message.error(`delete record failed: ${e}`);
        });
    }

    let toggleFavorite = (_: React.MouseEvent<HTMLElement>, doc: RecordDocument) => {

        invoke(CommandEnum.ToggleFavoriteRecordCommand, {
            viewId: doc.meta.viewId,
            oldFavorite: doc.fields.is_favorite,
        }).then(() => {
            setIsFavorite(old_state => {
                if (old_state === 0) {
                    return 1;
                } else {
                    return 0;
                }
            });
        }).catch(e => {
            message.error(`toggle record favorite failed: ${e}`);
        });
    }

    return (
        <div className={"record-card-right"}>
            <Button
                type={"text"}
                ghost={true}
                danger={true}
                size={"middle"}
                shape={"circle"}
                className={"record-favorite-button"}
                onClick={(e) => {
                    toggleFavorite(e, props.data)
                }}>
                {
                    isFavorite ? (<span>★</span>) : (<span>☆</span>)
                }
            </Button>
            <Button type={"text"}
                    ghost={true}
                    danger={true}
                    size={"middle"}
                    className={"record-delete-button"}
                    onClick={(e) => {
                        deleteRecord(e, props.data)
                    }
                    }>×</Button>
        </div>
    );
}
