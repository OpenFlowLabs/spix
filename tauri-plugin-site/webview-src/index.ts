import { invoke } from '@tauri-apps/api/tauri'

export type Site = {
  name: string,
  aliases: [string],
  files: [File],
}

export type File = {
  digest: Digest,
  asset_type: string,
}

export type Digest = {
  algo: string,
  digest_type: string,
  value: string,
}

export async function list_sites(): Promise<[string]> {
  return invoke('plugin:site|list_sites');
}

export async function create_site(site_name: string): Promise<Site> {
  return invoke('plugin:site|create_site', {site_name});
}

export async function save_site(site: Site): Promise<Site> {
  return invoke('plugin:site|save_site', {site});
}

export async function load_site(site_name: string): Promise<Site> {
  return invoke('plugin:site|load_site', {site_name})
}

export async function save_file(site: Site, file: string, asset_type: string): Promise<Site> {
  return invoke('plugin:site|save_file', {site, file, asset_type});
}

export async function load_file(site_name: string, file_digest: string): Promise<string> {
  return invoke('plugin:site|load_file', {site_name, file_digest});
}
