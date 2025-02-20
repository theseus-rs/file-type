use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117835119: FileType = FileType {
    file_format: &FileFormat {
        id: 117_835_119,
        source_type: SourceType::Wikidata,
        name: "Complete PC FAX/Portable file",
        extensions: &["cfp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
