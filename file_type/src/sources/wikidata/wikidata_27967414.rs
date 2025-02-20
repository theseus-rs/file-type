use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
