use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28055391: FileType = FileType {
    file_format: &FileFormat {
        id: 28_055_391,
        source_type: SourceType::Wikidata,
        name: "Printfox bitmap",
        extensions: &["bin", "bs", "gb", "pg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
