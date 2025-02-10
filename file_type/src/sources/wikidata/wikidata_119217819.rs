use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119217819: FileType = FileType {
    file_format: &FileFormat {
        id: 119_217_819,
        source_type: SourceType::Wikidata,
        name: "QuickBooks Portable Company File",
        extensions: &["qbm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
