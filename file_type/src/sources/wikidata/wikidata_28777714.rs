use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28777714: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_714,
        source_type: SourceType::Wikidata,
        name: "NII",
        extensions: &["nii"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
