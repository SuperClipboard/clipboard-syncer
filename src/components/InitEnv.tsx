import React, { useEffect, useState } from 'react';

type Props = {
    children: JSX.Element;
};

export const InitEnv: React.FC<Props> = ({ children }) => {
    const [ready, setReady] = useState(false);

    useEffect(() => {
        const init = async () => {
            setTimeout(() => {
                setReady(true);
            }, 1000);
        };

        init();
    }, []);

    return ready ? children : null;
};
