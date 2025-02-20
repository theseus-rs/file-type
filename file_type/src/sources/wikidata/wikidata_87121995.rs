use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87121995: FileType = FileType {
    file_format: &FileFormat {
        id: 87_121_995,
        source_type: SourceType::Wikidata,
        name: "Open Financial Exchange 2.1.1",
        extensions: &["ofx", "qfx"],
        media_types: &["application/x-ofx"],
        signatures: &[],
        related_formats: &[],
    },
};
