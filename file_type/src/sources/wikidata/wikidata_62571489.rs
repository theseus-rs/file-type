use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62571489: FileType = FileType {
    file_format: &FileFormat {
        id: 62_571_489,
        source_type: SourceType::Wikidata,
        name: "Unicode Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
