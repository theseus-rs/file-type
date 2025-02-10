use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58632423: FileType = FileType {
    file_format: &FileFormat {
        id: 58_632_423,
        source_type: SourceType::Wikidata,
        name: "Corel R.A.V.E.",
        extensions: &["clk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
