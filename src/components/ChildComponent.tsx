import React, { useContext } from 'react';
import GlobalContext from "@/components/GlobalContext";

const ChildComponent: React.FC = () => {
    const { graphqlEndpoint } = useContext(GlobalContext);

    // 使用全局变量进行渲染或其他操作
    return (
        <div>
            {graphqlEndpoint && <p>{graphqlEndpoint}</p>}
        </div>
    );
};

export default ChildComponent;