use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116869035: FileType = FileType {
    file_format: &FileFormat {
        id: 116_869_035,
        source_type: SourceType::Wikidata,
        name: "Summitsoft Business Card",
        extensions: &["crd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
