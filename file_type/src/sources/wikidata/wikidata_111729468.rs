use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111729468: FileType = FileType {
    file_format: &FileFormat {
        id: 111_729_468,
        source_type: SourceType::Wikidata,
        name: "RegMon File",
        extensions: &["rgd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
