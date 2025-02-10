use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66309247: FileType = FileType {
    file_format: &FileFormat {
        id: 66_309_247,
        source_type: SourceType::Wikidata,
        name: "Access Database Runtime",
        extensions: &["accdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
