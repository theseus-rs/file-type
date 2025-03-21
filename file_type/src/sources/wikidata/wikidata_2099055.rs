use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2099055: FileType = FileType {
    file_format: &FileFormat {
        id: 2_099_055,
        source_type: SourceType::Wikidata,
        name: "Playstation Patch File",
        extensions: &["ppf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
