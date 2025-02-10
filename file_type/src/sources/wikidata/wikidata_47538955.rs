use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47538955: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_955,
        source_type: SourceType::Wikidata,
        name: "AutoLISP Menu Source File",
        extensions: &["mnl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
