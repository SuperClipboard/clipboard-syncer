import React from 'react';
import {Input} from "antd";
import "@/css/SearchBar.css"

export const SearchBar: React.FC = () => {
    return (
        <div className={"search-bar-container"}>
            <Input type="text" className={"search-bar"} placeholder="Search..."/>
        </div>
    );
};
