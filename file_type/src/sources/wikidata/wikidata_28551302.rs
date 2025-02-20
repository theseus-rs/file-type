use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551302: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_302,
        source_type: SourceType::Wikidata,
        name: "Adobe Contour File",
        extensions: &["shc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
