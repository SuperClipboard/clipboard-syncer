import {DefaultGraphqlEndpoint, GraphqlEndpointCommand} from "@/utils/consts";
import React, { createContext, useState, useEffect } from 'react';
import {invoke} from "@tauri-apps/api";

interface Context {
    graphqlEndpoint: string;
}

const GlobalContext = createContext<Context>({
    graphqlEndpoint: DefaultGraphqlEndpoint
});

type Props = {
    children: JSX.Element;
};

export const GlobalContextProvider: React.FC<Props> = ({ children }) => {
    const [graphqlEndpoint, setGraphqlEndpoint] = useState<string>(DefaultGraphqlEndpoint);

    useEffect(() => {
        async function fetchData() {
            const result = await invoke(GraphqlEndpointCommand) as string; // 调用异步函数以获取返回值
            setGraphqlEndpoint(result); // 使用返回值来初始化全局变量
        }

        fetchData();
    }, []);

    return (
        <GlobalContext.Provider value={{ graphqlEndpoint: graphqlEndpoint }}>
            {children}
        </GlobalContext.Provider>
    );
};

export default GlobalContext;
