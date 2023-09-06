import ImageData from "@/models/ImageData";

export function parseImageData(imageData: string): ImageData {
    return JSON.parse(imageData) as ImageData;
}

export function base64ToImage(imageBase64: string): Blob {
    // 将 Base64 转换为 Blob 对象
    const byteCharacters = atob(imageBase64);
    const byteArrays = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
        byteArrays[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteArrays);
    return new Blob([byteArray], {type: "image/png"});
}
