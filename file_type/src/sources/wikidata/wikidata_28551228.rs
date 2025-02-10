use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551228: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_228,
        source_type: SourceType::Wikidata,
        name: "Adobe Action File",
        extensions: &["atn"],
        media_types: &["application/x-photoshop"],
        signatures: &[],
        related_formats: &[],
    },
};
