use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110015976: FileType = FileType {
    file_format: &FileFormat {
        id: 110_015_976,
        source_type: SourceType::Wikidata,
        name: "MIG Graphics File",
        extensions: &["mig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
