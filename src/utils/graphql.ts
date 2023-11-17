import {gql, GraphQLClient} from 'graphql-request';
import {GraphqlEndpoint} from "@/utils/consts";
import {AllFavoriteRecordsWrapper, PageRecordResponse, PageRecordResponseWrapper} from "@/models/RecordDocument";

const client = new GraphQLClient("http://localhost:12020/graphql");

export async function allFavoriteRecords(): Promise<PageRecordResponse> {
    const query = gql`
        query {
            favorite_resp: all_record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f(
                orderBy: create_time,
                orderDirection: DESC,
                filter: {
                    is_favorite: {in: [1]}
                }
            ) {
                documents {
                    fields {
                        content
                        content_preview
                        data_type
                        md5
                        create_time
                        is_favorite
                        tags
                        latest_addr
                        is_deleted
                    }
                    meta {
                        documentId
                        viewId
                        owner
                    }
                }
                hasNextPage
                endCursor
                totalCount
            }
        }`;

    try {
        let resp = await client.request<AllFavoriteRecordsWrapper>(query, {});
        console.debug(resp);
        return resp.favorite_resp;
    } catch (error) {
        console.error(
            `Error: make graphql request failed to ${GraphqlEndpoint}, error: ${error}`,
        );
        return Promise.reject(error);
    }
}

export async function getRecordByPage(limit: number = 20, startCursor?: string, favoriteFilter?: Array<number>): Promise<PageRecordResponse> {
    if (!startCursor || startCursor.length <= 0) {
        return firstGetRecordByPage();
    } else {
        return otherGetRecordByPage(limit, startCursor, favoriteFilter);
    }
}

async function firstGetRecordByPage(limit: number = 20, favoriteFilter?: Array<number>): Promise<PageRecordResponse> {
    const query = gql`
        query RecordByPages(
            $limit: Int = 20,
            $favorite_filter: [Int!] = [0],
            $order_by: record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fOrderBy = create_time,
            $order_dir: OrderDirection = DESC,
        ) {
            resp: all_record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f(
                orderBy: $order_by,
                orderDirection: $order_dir,
                first: $limit,
                filter: {
                    is_favorite: {in: $favorite_filter}
                }
            ) {
                documents {
                    fields {
                        content
                        content_preview
                        data_type
                        md5
                        create_time
                        is_favorite
                        tags
                        latest_addr
                        is_deleted
                    }
                    meta {
                        documentId
                        viewId
                        owner
                    }
                }
                hasNextPage
                endCursor
                totalCount
            }
        }`;

    try {
        let resp = await client.request<PageRecordResponseWrapper>(query, {
            limit: limit,
            favorite_filter: favoriteFilter?.toString(),
        });
        console.debug(resp);
        return resp.resp;
    } catch (error) {
        console.error(
            `Error: make graphql request failed to ${GraphqlEndpoint}, error: ${error}`,
        );
        return Promise.reject(error);
    }
}

export async function otherGetRecordByPage(limit: number = 20, startCursor: string, favoriteFilter?: Array<number>): Promise<PageRecordResponse> {
    const query = gql`
        query RecordByPages(
            $limit: Int = 20,
            $start_cursor: Cursor,
            $favorite_filter: [Int!] = [0],
            $order_by: record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099fOrderBy = create_time,
            $order_dir: OrderDirection = DESC,
        ) {
            resp: all_record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f(
                orderBy: $order_by,
                orderDirection: $order_dir,
                first: $limit,
                after: $start_cursor,
                filter: {
                    is_favorite: {in: $favorite_filter}
                }
            ) {
                documents {
                    fields {
                        content
                        content_preview
                        data_type
                        md5
                        create_time
                        is_favorite
                        tags
                        latest_addr
                        is_deleted
                    }
                    meta {
                        documentId
                        viewId
                        owner
                    }
                }
                hasNextPage
                endCursor
                totalCount
            }
        }`;

    try {
        let resp = await client.request<PageRecordResponseWrapper>(query, {
            limit: limit,
            start_cursor: startCursor,
            favorite_filter: favoriteFilter,
        });
        console.log(resp);
        return resp.resp;
    } catch (error) {
        console.error(
            `Error: make graphql request failed to ${GraphqlEndpoint}, error: ${error}`,
        );
        return Promise.reject(error);
    }
}
