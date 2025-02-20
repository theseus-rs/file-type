use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122676287: FileType = FileType {
    file_format: &FileFormat {
        id: 122_676_287,
        source_type: SourceType::Wikidata,
        name: "AFX AutoFX",
        extensions: &["afx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
