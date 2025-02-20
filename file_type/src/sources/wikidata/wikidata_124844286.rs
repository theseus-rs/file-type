use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124844286: FileType = FileType {
    file_format: &FileFormat {
        id: 124_844_286,
        source_type: SourceType::Wikidata,
        name: "CyberLink MediaShow Project",
        extensions: &["mbp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
