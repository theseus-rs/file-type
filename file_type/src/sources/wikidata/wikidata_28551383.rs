use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28551383: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_383,
        source_type: SourceType::Wikidata,
        name: "Adobe Replace Color/Color Range File",
        extensions: &["axt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
