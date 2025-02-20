use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975631: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_631,
        source_type: SourceType::Wikidata,
        name: "Moray User Defined Object",
        extensions: &["udo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
