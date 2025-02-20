use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127327283: FileType = FileType {
    file_format: &FileFormat {
        id: 127_327_283,
        source_type: SourceType::Wikidata,
        name: "Ada file",
        extensions: &["adb"],
        media_types: &["text/x-ada"],
        signatures: &[],
        related_formats: &[],
    },
};
