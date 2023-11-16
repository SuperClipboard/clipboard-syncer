import React from 'react';
import '@/css/MainSpotlight.css';
import {SearchBar} from "@/components/SearchBar";
import RecordList from "@/components/RecordList";

export const MainSpotlight: React.FC = () => {
    return (
        <div className={"spotlight"}>
            <SearchBar/>
            <RecordList/>
        </div>
    );
};
