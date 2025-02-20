use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62664735: FileType = FileType {
    file_format: &FileFormat {
        id: 62_664_735,
        source_type: SourceType::Wikidata,
        name: "Wordperfect Secondary File, version 5.1 and 5.2",
        extensions: &["doc"],
        media_types: &["application/wordperfect5.1"],
        signatures: &[],
        related_formats: &[],
    },
};
