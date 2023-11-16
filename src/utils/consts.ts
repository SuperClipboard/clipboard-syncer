import {invoke} from "@tauri-apps/api";

export const GraphqlEndpointCommand = "graphql_endpoint";

export const DefaultGraphqlEndpoint = "http://localhost:12020/graphql";

export let GraphqlEndpoint: string;

export enum EventListenerEnum {
    ChangeClipboardBackend = "cbs://change-clipboard-backend",
    TapChangeClipboardFrontend = "cbs://tap-change-clipboard-frontend",
}

(async () => {
    GraphqlEndpoint = await invoke(GraphqlEndpointCommand) as string; // 使用 async/await 来初始化全局变量
    console.log(`Get graphql endpoint: ${GraphqlEndpoint}`); // 确保变量已经被正确初始化
    return GraphqlEndpoint;
})();
