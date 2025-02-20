use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_96147075: FileType = FileType {
    file_format: &FileFormat {
        id: 96_147_075,
        source_type: SourceType::Wikidata,
        name: "Wolfram machine learning format",
        extensions: &["wmlf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
