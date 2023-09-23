import React from 'react';
import '@/css/App.css';
import RecordCard from "@/components/RecordCard";
import RecordList from "@/components/RecordList";

function App() {
    return (
        <div className="App">
            <RecordList />
            {/*<RecordCard data={temp_text}/>*/}
            {/*<RecordCard data={temp_image}/>*/}
        </div>
    );
}

export default App;
