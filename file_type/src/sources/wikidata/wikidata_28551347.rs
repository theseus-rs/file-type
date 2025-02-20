use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551347: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_347,
        source_type: SourceType::Wikidata,
        name: "Adobe Halftone Screens File",
        extensions: &["ahs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
