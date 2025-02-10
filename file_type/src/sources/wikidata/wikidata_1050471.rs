use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1050471: FileType = FileType {
    file_format: &FileFormat {
        id: 1_050_471,
        source_type: SourceType::Wikidata,
        name: "Property list",
        extensions: &["plist"],
        media_types: &["application/x-plist"],
        signatures: &[],
        related_formats: &[],
    },
};
