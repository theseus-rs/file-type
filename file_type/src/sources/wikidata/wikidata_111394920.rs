use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111394920: FileType = FileType {
    file_format: &FileFormat {
        id: 111_394_920,
        source_type: SourceType::Wikidata,
        name: "Form File",
        extensions: &["fif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
