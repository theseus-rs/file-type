use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
