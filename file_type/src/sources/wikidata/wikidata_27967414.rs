use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967414: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_414,
        source_type: SourceType::Wikidata,
        name: "Instrument Bank",
        extensions: &["ibk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
