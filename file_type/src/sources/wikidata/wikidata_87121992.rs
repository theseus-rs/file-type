use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87121992: FileType = FileType {
    file_format: &FileFormat {
        id: 87_121_992,
        source_type: SourceType::Wikidata,
        name: "Open Financial Exchange 2.0.3",
        extensions: &["ofx", "qfx"],
        media_types: &["application/x-ofx"],
        signatures: &[],
        related_formats: &[],
    },
};
