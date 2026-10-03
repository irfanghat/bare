// --------------------------------------------------------
// Can a byte-range-oriented graph representation provide an
// efficient abstraction for both local random access and remote graph access?
//
//
//                     BARE
//                      │
//         ┌────────────┴────────────┐
//         │                         │
//    BARE File                  BARE Service
//         │                         │
//         │                    Apache Thrift
//         │                         │
//  ┌──────────────┐          ┌────────────────┐
//  │ Header       │          │ get_neighbors  │
//  │ Vertex Index │          │ get_range      │
//  │ Adjacencies  │          │ get_vertex     │
//  │ Metadata     │          │ scan_vertices  │
//  └──────────────┘          └────────────────┘
//
//
//      Client
//        │
//        │ Thrift
//  ┌─────────────┐
//  │ BARE Server │
//  └──────┬──────┘
//         │
//   BARE file
//         │
//  vertex index
//         │
//  offset & length
//         │
//  adjacency bytes
// --------------------------------------------------------

pub mod format;
pub mod writer;

use crate::format::{AdjacencyRange, Header};
