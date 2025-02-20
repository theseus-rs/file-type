use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59820771: FileType = FileType {
    file_format: &FileFormat {
        id: 59_820_771,
        source_type: SourceType::Wikidata,
        name: "Corel R.A.V.E.",
        extensions: &["clk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
