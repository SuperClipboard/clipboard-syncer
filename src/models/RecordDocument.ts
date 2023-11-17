import {Record} from "@/models/Record";

export type RecordDocument = {
    meta: Meta,
    fields: Record,
}

export type Meta = {
    viewId: string;
    documentId: string;
    owner: string;
};

export type PageRecordResponseWrapper = {
    resp: PageRecordResponse,
};

export type AllFavoriteRecordsWrapper = {
    favorite_resp: PageRecordResponse,
}

export type PageRecordResponse = {
    documents: RecordDocument[],
    hasNextPage: boolean,
    endCursor: string,
    totalCount: number,
}

