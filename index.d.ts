/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface ConnectionOptions {
  resourceArn: string
  secretArn: string
  database: string
  region: string
}
export interface IncludeField {
  joins: Array<Join>
}
export interface Join {
  table: string
  key: string
  joiningTable: string
  joiningKey: string
}
export interface Field {
  name: string
  arguments: Array<Argument>
}
export interface Argument {
  name: string
  value?: string
  valueType?: string
}
export interface OrderBy {
  field: string
  order: Order
}
export const enum Order {
  Asc = 0,
  Desc = 1
}
export interface Options {
  orderBy?: Array<OrderBy>
  limit?: number
  offset?: number
  numOfRows?: number
  include?: IncludeField
  groupBy?: Array<string>
}
export interface Metadata {
  command: string
  table: string
  schema: string
}
export interface Statement {
  metadata: Metadata
  fields: Array<Field>
  options: Options
}
export function connect(): Promise<void>
export function introspect(): Promise<string>
export function getForeignKeysData(table: string): Promise<string>
export function query(fields: Array<Field>, options: Options, metadata: Metadata): Promise<string | null>
export function queryRaw(queryString: string): Promise<string | null>
export interface ConnectionOptions {
  resourceArn: string
  secretArn: string
  database: string
  region: string
}
export interface IncludeField {
  joins: Array<Join>
}
export interface Join {
  table: string
  key: string
  joiningTable: string
  joiningKey: string
}
export interface Field {
  name: string
  arguments: Array<Argument>
}
export interface Argument {
  name: string
  value?: string
  valueType?: string
}
export interface OrderBy {
  field: string
  order: Order
}
export const enum Order {
  Asc = 0,
  Desc = 1
}
export interface Options {
  orderBy?: Array<OrderBy>
  limit?: number
  offset?: number
  numOfRows?: number
  include?: IncludeField
  groupBy?: Array<string>
}
export interface Metadata {
  command: string
  table: string
  schema: string
}
export interface Statement {
  metadata: Metadata
  fields: Array<Field>
  options: Options
}
export function connect(): Promise<void>
export function introspect(): Promise<string>
export function getForeignKeysData(table: string): Promise<string>
export function query(fields: Array<Field>, options: Options, metadata: Metadata): Promise<string | null>
export function queryRaw(queryString: string): Promise<string | null>
