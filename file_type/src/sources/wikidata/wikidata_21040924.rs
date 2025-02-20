use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21040924: FileType = FileType {
    file_format: &FileFormat {
        id: 21_040_924,
        source_type: SourceType::Wikidata,
        name: "NoiseTrekker format",
        extensions: &["ntk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
