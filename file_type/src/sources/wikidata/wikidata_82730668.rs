use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_82730668: FileType = FileType {
    file_format: &FileFormat {
        id: 82_730_668,
        source_type: SourceType::Wikidata,
        name: "Microsoft Entourage Archive",
        extensions: &["rge"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
