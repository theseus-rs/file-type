use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129571499: FileType = FileType {
    file_format: &FileFormat {
        id: 129_571_499,
        source_type: SourceType::Wikidata,
        name: "HSAIL assembly code file",
        extensions: &["hsail"],
        media_types: &["text/x-hsail"],
        signatures: &[],
        related_formats: &[],
    },
};
