export const RecordDataTypeEnum = {
    Text: "text",
    Image: "image",
}

export type Record = {
    content: string,
    content_preview: string,
    // data_type(文本=text、图片=image)
    data_type: string,
    md5: string,
    create_time: number,
    is_favorite: number,
    tags: string[],
    latest_addr: string,
    is_deleted: number,
}
