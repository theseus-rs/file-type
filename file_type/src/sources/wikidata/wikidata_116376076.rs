use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116376076: FileType = FileType {
    file_format: &FileFormat {
        id: 116_376_076,
        source_type: SourceType::Wikidata,
        name: "Access Database Addins",
        extensions: &["mda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
