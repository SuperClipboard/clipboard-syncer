import React from 'react';
import '@/css/App.css';
import RecordCard from "@/components/RecordCard";
import {temp_image, temp_text} from "@/models/Record";

function App() {
    return (
        <div className="App">
            {/*<RecordList />*/}
            <RecordCard data={temp_text}/>
            <RecordCard data={temp_image}/>
        </div>
    );
}

export default App;
