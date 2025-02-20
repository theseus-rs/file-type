use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132156163: FileType = FileType {
    file_format: &FileFormat {
        id: 132_156_163,
        source_type: SourceType::Wikidata,
        name: "NIMAS XML file format",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
