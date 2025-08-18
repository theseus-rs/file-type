use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135318359: FileType = FileType {
    file_format: &FileFormat {
        id: 135_318_359,
        source_type: SourceType::Wikidata,
        name: "Q135318359",
        extensions: &["KCMSF"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
