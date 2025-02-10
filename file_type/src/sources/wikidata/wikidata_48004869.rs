use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48004869: FileType = FileType {
    file_format: &FileFormat {
        id: 48_004_869,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Database, version 95",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
