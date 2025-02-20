use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111331475: FileType = FileType {
    file_format: &FileFormat {
        id: 111_331_475,
        source_type: SourceType::Wikidata,
        name: "Mus10 audio file",
        extensions: &["mus10"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
