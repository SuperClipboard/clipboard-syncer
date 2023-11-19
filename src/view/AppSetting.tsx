import React, {useEffect} from "react";
import {Button, Form, Input, InputNumber, message} from "antd";
import "@/css/AppSetting.css";
import {useNavigate} from 'react-router-dom';
import {AppConfig} from "@/models/AppConfig";
import {invoke} from "@tauri-apps/api";
import {CommandEnum} from "@/utils/consts";
import { ValidateErrorEntity } from "rc-field-form/lib/interface";

export default function AppSetting() {

    const navigate = useNavigate();
    const [formData] = Form.useForm<AppConfig>();

    useEffect(() => {
        loadConfig();
    }, []);

    const loadConfig = () => {

        invoke<AppConfig>(CommandEnum.LoadAppConfigCommand).then(oldConfig => {
            console.log(`Load config success: ${oldConfig.sync_port}`);
            formData.setFieldsValue(oldConfig);
        }).catch(e => {
            message.error(`change clipboard record failed: ${e}`);
        });
    };

    const handleSubmit = async (values: AppConfig) => {
        try {
            await invoke(CommandEnum.SaveAppConfigCommand, {
                config: values
            });
            navigate("/");
        } catch (e) {
            message.error(`内部错误：${e}`);
        }
    };

    const handleSubmitFailed = (errorInfo: ValidateErrorEntity<AppConfig>) => {
        console.log('Failed:', errorInfo);
    };

    const handleCancel = () => {
        console.log("Cancel clicked!");
        navigate("/");
    }


    return (
        <div className={"setting-container"}>
            <div className={"setting-title"}>设置</div>
            <div>
                {formData.getFieldValue.name}
            </div>

            <Form
                className={"setting-form"}
                form={formData}
                name="basic"
                layout={"vertical"}
                labelCol={{span: 8}}
                wrapperCol={{span: 16}}
                style={{maxWidth: 600}}
                initialValues={{remember: true}}
                onFinish={handleSubmit}
                onFinishFailed={handleSubmitFailed}
                autoComplete="off"
            >
                <Form.Item<AppConfig>
                    label="剪切板最大储存数量"
                    name="store_limit"
                    rules={[{type: "number", required: true, min: 1, max: 100000}]}
                >
                    <InputNumber/>
                </Form.Item>
                <Form.Item<AppConfig>
                    label="数据同步端口号（重启后生效）"
                    name="sync_port"
                    rules={[{type: "number", required: true, min: 1, max: 65535}]}
                >
                    <InputNumber/>
                </Form.Item>
                <Form.Item<AppConfig>
                    label="GraghQL服务端口号（重启后生效）"
                    name="graphql_port"
                    rules={[{type: "number", required: true, min: 1, max: 65535}]}
                >
                    <InputNumber/>
                </Form.Item>
                <Form.Item<AppConfig>
                    label="打开/关闭窗口快捷键（Linux暂无法使用）"
                    name="toggle_window_hotkey"
                    rules={[{type: "string", required: true}]}
                >
                    <Input disabled={true}/>
                </Form.Item>

                <Form.Item wrapperCol={{offset: 8, span: 16}}>
                    <Button htmlType="submit"
                            className={"setting-btn-submit"}
                            type={"primary"}
                            size={"middle"}>
                        保存
                    </Button>
                    <Button className={"setting-btn-cancel"}
                            type={"primary"}
                            size={"middle"}
                            onClick={handleCancel}>
                        取消
                    </Button>
                </Form.Item>
            </Form>
        </div>
    )
}
