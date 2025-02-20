use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4051789: FileType = FileType {
    file_format: &FileFormat {
        id: 4_051_789,
        source_type: SourceType::Wikidata,
        name: "Trivial Graph Format",
        extensions: &["tgf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
