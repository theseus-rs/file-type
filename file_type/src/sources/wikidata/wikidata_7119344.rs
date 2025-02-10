use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7119344: FileType = FileType {
    file_format: &FileFormat {
        id: 7_119_344,
        source_type: SourceType::Wikidata,
        name: "PICtor PIC image format",
        extensions: &["clp", "pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
