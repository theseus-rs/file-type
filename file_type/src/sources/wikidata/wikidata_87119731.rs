use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87119731: FileType = FileType {
    file_format: &FileFormat {
        id: 87_119_731,
        source_type: SourceType::Wikidata,
        name: "Open Financial Exchange 1.02",
        extensions: &["ofx", "qfx"],
        media_types: &["application/x-ofx"],
        signatures: &[],
        related_formats: &[],
    },
};
