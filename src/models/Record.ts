import {invoke} from "@tauri-apps/api";
import {find_records_by_pages_command} from "@/utils/consts";

export enum RecordDataTypeEnum {
    Text = "text",
    Image = "image",
}

export default interface Record {
    id: number,
    content: string,
    content_preview: string,
    // data_type(文本=text、图片=image)
    data_type: string,
    md5: string,
    create_time: number,
    is_favorite: number,
    tags: string[],
    latest_addr: string,
}
