
/**
 * @roxi/routify 2.18.12
 * File generated Wed Jul 24 2024 18:39:25 GMT-0400 (Eastern Daylight Time)
 */

export const __version = "2.18.12"
export const __timestamp = "2024-07-24T22:39:25.912Z"

//buildRoutes
import { buildClientTree } from "@roxi/routify/runtime/buildRoutes"

//imports


//options
export const options = {}

//tree
export const _tree = {
  "root": true,
  "children": [
    {
      "isDir": true,
      "ext": "",
      "children": [
        {
          "isIndex": true,
          "isPage": true,
          "path": "/characters/index",
          "id": "_characters_index",
          "component": () => import('../src/pages/characters/index.svelte').then(m => m.default)
        }
      ],
      "path": "/characters"
    },
    {
      "isDir": true,
      "ext": "",
      "children": [
        {
          "isIndex": true,
          "isPage": true,
          "path": "/chat/index",
          "id": "_chat_index",
          "component": () => import('../src/pages/chat/index.svelte').then(m => m.default)
        }
      ],
      "path": "/chat"
    },
    {
      "isDir": true,
      "ext": "",
      "children": [
        {
          "isIndex": true,
          "isPage": true,
          "path": "/edit_character/index",
          "id": "_edit_character_index",
          "component": () => import('../src/pages/edit_character/index.svelte').then(m => m.default)
        }
      ],
      "path": "/edit_character"
    },
    {
      "isIndex": true,
      "isPage": true,
      "path": "/index",
      "id": "_index",
      "component": () => import('../src/pages/index.svelte').then(m => m.default)
    }
  ],
  "isLayout": true,
  "path": "/",
  "id": "__layout",
  "component": () => import('../src/pages/_layout.svelte').then(m => m.default)
}


export const {tree, routes} = buildClientTree(_tree)

