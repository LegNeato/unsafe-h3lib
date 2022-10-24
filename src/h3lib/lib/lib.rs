#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate libc;
/*

pub mod apps {
    pub mod applib {
        pub mod lib {
            pub mod args;
            pub mod kml;
            pub mod test;
            pub mod utility;
        } // mod lib
    } // mod applib
    pub mod benchmarks {
        pub mod benchmarkCellsToLinkedMultiPolygon;
        pub mod benchmarkDirectedEdge;
        pub mod benchmarkGridDiskCells;
        pub mod benchmarkGridPathCells;
        pub mod benchmarkH3Api;
        pub mod benchmarkIsValidCell;
        pub mod benchmarkPolygon;
        pub mod benchmarkPolygonToCells;
        pub mod benchmarkVertex;
    } // mod benchmarks
    pub mod filters {
        pub mod cellToBoundary;
        pub mod cellToLatLng;
        pub mod cellToLocalIj;
        pub mod gridDisk;
        pub mod gridDiskUnsafe;
        pub mod h3;
        pub mod h3ToComponents;
        pub mod latLngToCell;
        pub mod localIjToCell;
    } // mod filters
    pub mod fuzzers {
        pub mod fuzzerCellArea;
        pub mod fuzzerCellProperties;
        pub mod fuzzerCellToLatLng;
        pub mod fuzzerCellsToLinkedMultiPolygon;
        pub mod fuzzerCompact;
        pub mod fuzzerDirectedEdge;
        pub mod fuzzerDistances;
        pub mod fuzzerEdgeLength;
        pub mod fuzzerGridDisk;
        pub mod fuzzerHierarchy;
        pub mod fuzzerIndexIO;
        pub mod fuzzerInternalAlgos;
        pub mod fuzzerLatLngToCell;
        pub mod fuzzerLocalIj;
        pub mod fuzzerPolygonToCells;
        pub mod fuzzerPolygonToCellsNoHoles;
        pub mod fuzzerResolutions;
        pub mod fuzzerVertexes;
    } // mod fuzzers
    pub mod miscapps {
        pub mod cellToBoundaryHier;
        pub mod cellToLatLngHier;
        pub mod generateBaseCellNeighbors;
        pub mod generateFaceCenterPoint;
        pub mod generatePentagonDirectionFaces;
        pub mod h3ToHier;
    } // mod miscapps
    pub mod testapps {
        pub mod mkRandGeo;
        pub mod mkRandGeoBoundary;
        pub mod testBBox;
        pub mod testBaseCells;
        pub mod testCellToBoundary;
        pub mod testCellToBoundaryEdgeCases;
        pub mod testCellToCenterChild;
        pub mod testCellToChildren;
        pub mod testCellToChildrenSize;
        pub mod testCellToLatLng;
        pub mod testCellToLocalIj;
        pub mod testCellToLocalIjExhaustive;
        pub mod testCellToParent;
        pub mod testCellsToLinkedMultiPolygon;
        pub mod testCompactCells;
        pub mod testCoordIj;
        pub mod testCoordIjk;
        pub mod testDirectedEdge;
        pub mod testDirectedEdgeExhaustive;
        pub mod testGetIcosahedronFaces;
        pub mod testGridDisk;
        pub mod testGridDisksUnsafe;
        pub mod testGridDistance;
        pub mod testGridDistanceExhaustive;
        pub mod testGridPathCells;
        pub mod testGridPathCellsExhaustive;
        pub mod testGridRingUnsafe;
        pub mod testH3Api;
        pub mod testH3CellArea;
        pub mod testH3CellAreaExhaustive;
        pub mod testH3Index;
        pub mod testH3Iterators;
        pub mod testH3Memory;
        pub mod testH3NeighborRotations;
        pub mod testH3SetToVertexGraph;
        pub mod testLatLng;
        pub mod testLatLngToCell;
        pub mod testLinkedGeo;
        pub mod testPentagonIndexes;
        pub mod testPolygon;
        pub mod testPolygonToCells;
        pub mod testPolygonToCellsReported;
        pub mod testVec2d;
        pub mod testVec3d;
        pub mod testVertex;
        pub mod testVertexExhaustive;
        pub mod testVertexGraph;
    } // mod testapps
} // mod apps
        */

pub mod algos;
pub mod baseCells;
pub mod bbox;
pub mod coordijk;
pub mod directedEdge;
pub mod faceijk;
pub mod h3Index;
pub mod iterators;
pub mod latLng;
pub mod linkedGeo;
pub mod localij;
pub mod mathExtensions;
pub mod polygon;
pub mod vec2d;
pub mod vec3d;
pub mod vertex;
pub mod vertexGraph;
