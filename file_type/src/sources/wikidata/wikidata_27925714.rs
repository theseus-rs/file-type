use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27925714: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_714,
        source_type: SourceType::Wikidata,
        name: "DTED Level 1 Gazetteer Hash file",
        extensions: &["hsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
