use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112669255: FileType = FileType {
    file_format: &FileFormat {
        id: 112_669_255,
        source_type: SourceType::Wikidata,
        name: "JSR-184 format",
        extensions: &["m3g"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
