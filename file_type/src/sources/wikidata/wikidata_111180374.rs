use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111180374: FileType = FileType {
    file_format: &FileFormat {
        id: 111_180_374,
        source_type: SourceType::Wikidata,
        name: "PressWriter File",
        extensions: &["psp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
