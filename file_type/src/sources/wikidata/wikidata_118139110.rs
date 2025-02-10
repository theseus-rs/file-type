use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118139110: FileType = FileType {
    file_format: &FileFormat {
        id: 118_139_110,
        source_type: SourceType::Wikidata,
        name: "Calendar Creator 2.x Event File",
        extensions: &["cee"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
