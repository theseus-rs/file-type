use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650514: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_514,
        source_type: SourceType::Wikidata,
        name: "packPNM",
        extensions: &["ppn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
