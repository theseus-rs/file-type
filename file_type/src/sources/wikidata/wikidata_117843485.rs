use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117843485: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_485,
        source_type: SourceType::Wikidata,
        name: "Faxable PCX",
        extensions: &["fcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
