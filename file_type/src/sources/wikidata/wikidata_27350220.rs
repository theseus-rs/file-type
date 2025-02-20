use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27350220: FileType = FileType {
    file_format: &FileFormat {
        id: 27_350_220,
        source_type: SourceType::Wikidata,
        name: "ADRG General Information File",
        extensions: &["gen"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
