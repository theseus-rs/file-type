use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127378208: FileType = FileType {
    file_format: &FileFormat {
        id: 127_378_208,
        source_type: SourceType::Wikidata,
        name: "FreeBASIC source code file",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
