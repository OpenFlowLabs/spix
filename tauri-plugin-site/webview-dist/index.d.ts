export declare type Site = {
    name: string;
    aliases: [string];
    files: [File];
};
export declare type File = {
    digest: Digest;
    asset_type: string;
};
export declare type Digest = {
    algo: string;
    digest_type: string;
    value: string;
};
export declare function list_sites(): Promise<[string]>;
export declare function create_site(site_name: string): Promise<Site>;
export declare function save_site(site: Site): Promise<Site>;
export declare function load_site(site_name: string): Promise<Site>;
export declare function save_file(site: Site, file: string, asset_type: string): Promise<Site>;
export declare function load_file(site_name: string, file_digest: string): Promise<string>;
