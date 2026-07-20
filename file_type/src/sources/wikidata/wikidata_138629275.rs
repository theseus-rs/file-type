use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138629275: FileType = FileType {
    file_format: &FileFormat {
        id: 138_629_275,
        source_type: SourceType::Wikidata,
        name: "Extended Gerber format",
        extensions: &["gbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
