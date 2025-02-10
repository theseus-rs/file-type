use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_19969536: FileType = FileType {
    file_format: &FileFormat {
        id: 19_969_536,
        source_type: SourceType::Wikidata,
        name: "DSV version 6 format",
        extensions: &["dsv", "dsv6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
