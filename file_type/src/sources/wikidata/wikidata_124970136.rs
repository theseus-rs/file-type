use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124970136: FileType = FileType {
    file_format: &FileFormat {
        id: 124_970_136,
        source_type: SourceType::Wikidata,
        name: "MIX status file",
        extensions: &["mixstatus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
