use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134705363: FileType = FileType {
    file_format: &FileFormat {
        id: 134_705_363,
        source_type: SourceType::Wikidata,
        name: "OpenOffice.org 1.0 Text Document",
        extensions: &["swx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
