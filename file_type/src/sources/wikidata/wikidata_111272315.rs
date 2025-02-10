use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272315: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_315,
        source_type: SourceType::Wikidata,
        name: "Ensoniq SQ80 instrument file",
        extensions: &["efs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
