use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5421818: FileType = FileType {
    file_format: &FileFormat {
        id: 5_421_818,
        source_type: SourceType::Wikidata,
        name: "Extended Log Format",
        extensions: &["log"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
