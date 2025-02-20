use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_46118844: FileType = FileType {
    file_format: &FileFormat {
        id: 46_118_844,
        source_type: SourceType::Wikidata,
        name: "Lotus Freelance Smartmaster Graphics",
        extensions: &["mas"],
        media_types: &["application/vnd.lotus-freelance"],
        signatures: &[],
        related_formats: &[],
    },
};
