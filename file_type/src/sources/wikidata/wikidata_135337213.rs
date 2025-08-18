use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135337213: FileType = FileType {
    file_format: &FileFormat {
        id: 135_337_213,
        source_type: SourceType::Wikidata,
        name: "IBM MQ script",
        extensions: &["mqsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
