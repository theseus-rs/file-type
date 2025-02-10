use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111519671: FileType = FileType {
    file_format: &FileFormat {
        id: 111_519_671,
        source_type: SourceType::Wikidata,
        name: "PageMaker template file, version 5",
        extensions: &["pt5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
