use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551319: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_319,
        source_type: SourceType::Wikidata,
        name: "Adobe Custom Kernel File",
        extensions: &["acf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
