use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167864: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_864,
        source_type: SourceType::Wikidata,
        name: "Pittsburgh Supercomputing Center 3D Metafile",
        extensions: &["p3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
