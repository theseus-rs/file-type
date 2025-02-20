use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111317248: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_248,
        source_type: SourceType::Wikidata,
        name: "Korg Triton or Trinity script file",
        extensions: &["ksc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
