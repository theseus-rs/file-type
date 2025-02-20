use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124970024: FileType = FileType {
    file_format: &FileFormat {
        id: 124_970_024,
        source_type: SourceType::Wikidata,
        name: "MIX metadata file",
        extensions: &["mixmeta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
