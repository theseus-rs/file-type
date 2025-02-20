use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87121491: FileType = FileType {
    file_format: &FileFormat {
        id: 87_121_491,
        source_type: SourceType::Wikidata,
        name: "Open Financial Exchange 1.6",
        extensions: &["ofx", "qfx"],
        media_types: &["application/x-ofx"],
        signatures: &[],
        related_formats: &[],
    },
};
