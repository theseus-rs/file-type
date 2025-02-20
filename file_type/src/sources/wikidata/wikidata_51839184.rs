use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51839184: FileType = FileType {
    file_format: &FileFormat {
        id: 51_839_184,
        source_type: SourceType::Wikidata,
        name: "Ventura Publisher",
        extensions: &["gen"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
