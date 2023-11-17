import "@/css/RecoardCard.css"
import React, {useState} from "react";
import {Button, Image, message} from "antd";
import {base64ToImage, parseImageData} from "@/utils/image";
import {CommandEnum} from "@/utils/consts";
import {Record, RecordDataTypeEnum} from "@/models/Record";
import {Base64} from "js-base64";
import {RecordDocument} from "@/models/RecordDocument";
import {invoke} from "@tauri-apps/api";

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

        let content = record.content;
        if (record.data_type === RecordDataTypeEnum.Image) {
            content = btoa(content);
        } else if (record.data_type === RecordDataTypeEnum.Text) {
            content = Base64.encode(content);
        }

        invoke(CommandEnum.TapChangeClipboardCommand, {
            content: content,
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
                    <p>{props.data.fields.content_preview}</p>
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
