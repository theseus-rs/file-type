use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_97359795: FileType = FileType {
    file_format: &FileFormat {
        id: 97_359_795,
        source_type: SourceType::Wikidata,
        name: "AnIML",
        extensions: &["animl"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
