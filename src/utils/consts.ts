import {invoke} from "@tauri-apps/api";

export enum CommandEnum {
    GraphqlEndpointCommand = "graphql_endpoint",
    TapChangeClipboardCommand = "tap_change_clipboard",
    DeleteRecordCommand = "delete_record",
    ToggleFavoriteRecordCommand = "toggle_favorite_record",
}

export enum EventListenerEnum {
    ChangeClipboardBackend = "cbs://change-clipboard-backend",
    UpdateClipboardRecordBackend = "cbs://update-clipboard-record-backend",
    DeleteClipboardRecordBackend = "cbs://delete-clipboard-record-backend",
}

export const DefaultGraphqlEndpoint = "http://localhost:12020/graphql";

export let GraphqlEndpoint: string;

(async () => {
    GraphqlEndpoint = await invoke(CommandEnum.GraphqlEndpointCommand) as string; // 使用 async/await 来初始化全局变量
    console.log(`Get graphql endpoint: ${GraphqlEndpoint}`); // 确保变量已经被正确初始化
    return GraphqlEndpoint;
})();
