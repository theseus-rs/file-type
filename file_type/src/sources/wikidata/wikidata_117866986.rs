use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117866986: FileType = FileType {
    file_format: &FileFormat {
        id: 117_866_986,
        source_type: SourceType::Wikidata,
        name: "American Data Tech SMARTFAX file",
        extensions: &["smf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
