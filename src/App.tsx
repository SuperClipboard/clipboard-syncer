import React from 'react';
import '@/css/App.css';
import {GlobalContextProvider} from "@/components/GlobalContext";
import {InitEnv} from "@/components/InitEnv";
import {MainSpotlight} from "@/view/MainSpotlight";
import {Route, Routes} from "react-router-dom";
import AppSetting from "@/view/AppSetting";

function App() {
    return (
        <InitEnv>
            <GlobalContextProvider>
                <div className="App">
                    <Routes>
                        <Route path={"/"} element={<MainSpotlight/>}/>
                        <Route path={"/setting"} element={<AppSetting/>}/>
                    </Routes>
                </div>
            </GlobalContextProvider>
        </InitEnv>
    );
}

export default App;
