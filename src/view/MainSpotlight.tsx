import React, {ChangeEvent, useCallback, useState} from 'react';
import '@/css/MainSpotlight.css';
import RecordList from "@/components/RecordList";
import {Button, Input} from "antd";
import {debounce} from "lodash";
import {SettingOutlined} from "@ant-design/icons";
import {Link} from "react-router-dom";

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

    const navigate_to_settings = (e: React.MouseEvent<HTMLElement>) => {
        console.log(`navigate_to_settings: ${e}`)
    };

    return (
        <div className={"spotlight"}>
            <div className={"search-container"}>
                <Input
                    type="text"
                    className={"search-bar"}
                    value={searchKeyword}
                    onChange={e => handleKeywordChange(e)}
                    placeholder="Search..."/>

                <Link to={{pathname: "/setting"}} replace={true}>
                    <Button className={"search-bar-end"}
                            type={"text"}
                            size={"middle"}
                            shape={"circle"}
                            onClick={(e) => {
                                navigate_to_settings(e);
                            }}
                            icon={<SettingOutlined />}/>
                </Link>
            </div>
            <RecordList searchKeyword={searchKeywordChild}/>
        </div>
    );
};
