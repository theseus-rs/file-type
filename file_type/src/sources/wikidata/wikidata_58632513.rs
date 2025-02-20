use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58632513: FileType = FileType {
    file_format: &FileFormat {
        id: 58_632_513,
        source_type: SourceType::Wikidata,
        name: "Corel R.A.V.E.",
        extensions: &["clk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
