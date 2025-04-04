use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551355: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_355,
        source_type: SourceType::Wikidata,
        name: "Adobe Hue/Saturation File",
        extensions: &["ahu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
