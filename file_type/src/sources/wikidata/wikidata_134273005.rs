use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134273005: FileType = FileType {
    file_format: &FileFormat {
        id: 134_273_005,
        source_type: SourceType::Wikidata,
        name: "Clipper header file",
        extensions: &["ch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
