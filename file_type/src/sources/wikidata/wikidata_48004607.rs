use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48004607: FileType = FileType {
    file_format: &FileFormat {
        id: 48_004_607,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Database, version 2",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
