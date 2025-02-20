use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47916123: FileType = FileType {
    file_format: &FileFormat {
        id: 47_916_123,
        source_type: SourceType::Wikidata,
        name: "Frame Vector Metafile",
        extensions: &["fmv"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
