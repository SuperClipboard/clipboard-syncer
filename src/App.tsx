import React from 'react';
import '@/css/App.css';
import {GlobalContextProvider} from "@/components/GlobalContext";
import RecordList from "@/components/RecordList";
import {InitEnv} from "@/components/InitEnv";

function App() {
    return (
        <InitEnv>
            <GlobalContextProvider>
                <div className="App">
                    {/*<ChildComponent/>*/}
                    <RecordList/>
                    {/*<RecordCard data={temp_text}/>*/}
                    {/*<RecordCard data={temp_image}/>*/}
                </div>
            </GlobalContextProvider>
        </InitEnv>
    );
}

export default App;
