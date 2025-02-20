use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125298126: FileType = FileType {
    file_format: &FileFormat {
        id: 125_298_126,
        source_type: SourceType::Wikidata,
        name: "Scheme library definition",
        extensions: &["sld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
