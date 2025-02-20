use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
