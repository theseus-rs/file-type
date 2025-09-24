use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135982741: FileType = FileType {
    file_format: &FileFormat {
        id: 135_982_741,
        source_type: SourceType::Wikidata,
        name: "Semblio file format",
        extensions: &["semblio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
