import React from 'react';
import '@/css/App.css';
import {GlobalContextProvider} from "@/components/GlobalContext";
import {InitEnv} from "@/components/InitEnv";
import {MainSpotlight} from "@/view/MainSpotlight";

function App() {
    return (
        <InitEnv>
            <GlobalContextProvider>
                <div className="App">
                    <MainSpotlight/>
                </div>
            </GlobalContextProvider>
        </InitEnv>
    );
}

export default App;
