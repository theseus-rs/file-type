use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951550: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_550,
        source_type: SourceType::Wikidata,
        name: "J script file",
        extensions: &["ijs"],
        media_types: &["text/x-j"],
        signatures: &[],
        related_formats: &[],
    },
};
