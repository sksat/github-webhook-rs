export type Schema =
  | IssueCommentEvent
  | IssuesEvent

export type IssueCommentEvent =
  | IssueCommentCreatedEvent
  | IssueCommentDeletedEvent
  | IssueCommentEditedEvent;

export interface User {
  login: string;
  id: number;
  node_id: string;
  name?: string;
  email?: string | null;
  avatar_url: string;
  received_events_url: string;
  type: "Bot" | "User" | "Organization";
  site_admin: boolean;
}
export interface License {
  key: string;
  name: string;
  spdx_id: string;
  url: string | null;
  node_id: string;
}

export type WebhookEvent = Schema;

export interface Label {
  id: number;
  node_id: string;
  url: string;
  name: string;
  description: string | null;
  color: string;
  default: boolean;
}


export interface Reactions {
  url: string;
  total_count: number;
  "+1": number;
  "-1": number;
  laugh: number;
  hooray: number;
  confused: number;
  heart: number;
  rocket: number;
  eyes: number;
}
