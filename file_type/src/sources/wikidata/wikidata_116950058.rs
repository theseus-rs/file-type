use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116950058: FileType = FileType {
    file_format: &FileFormat {
        id: 116_950_058,
        source_type: SourceType::Wikidata,
        name: "Ulead COOL 360 Project File",
        extensions: &["upj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
