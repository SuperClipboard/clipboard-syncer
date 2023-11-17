import React, {ChangeEvent, useCallback, useState} from 'react';
import '@/css/MainSpotlight.css';
import RecordList from "@/components/RecordList";
import {Input} from "antd";
import {debounce} from "lodash";

export const MainSpotlight: React.FC = () => {

    const [searchKeyword, setSearchKeyword] = useState("");
    const [searchKeywordChild, setSearchKeywordChild] = useState("")

    const handleDebounceSearch = useCallback(debounce((value) => {
        console.log(`searchKeyword changed: ${value}`);
        setSearchKeywordChild(value);
    }, 600), []);

    const handleKeywordChange = async (e: ChangeEvent<HTMLInputElement>) => {
        const {value} = e.target;
        setSearchKeyword(value)
        handleDebounceSearch(value);
    }

    return (
        <div className={"spotlight"}>
            <Input
                type="text"
                className={"search-bar"}
                value={searchKeyword}
                onChange={e => handleKeywordChange(e)}
                placeholder="Search..."/>
            <RecordList searchKeyword={searchKeywordChild}/>
        </div>
    );
};
