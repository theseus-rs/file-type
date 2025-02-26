use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132635828: FileType = FileType {
    file_format: &FileFormat {
        id: 132_635_828,
        source_type: SourceType::Wikidata,
        name: "PL/M Source Code File",
        extensions: &["plm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
