use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47895228: FileType = FileType {
    file_format: &FileFormat {
        id: 47_895_228,
        source_type: SourceType::Wikidata,
        name: "Visual Basic Macro",
        extensions: &["dvb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
