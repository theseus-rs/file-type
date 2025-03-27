use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111147971: FileType = FileType {
    file_format: &FileFormat {
        id: 111_147_971,
        source_type: SourceType::Wikidata,
        name: "General Purpose Raw",
        extensions: &["gpr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
