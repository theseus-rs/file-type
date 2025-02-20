use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48005022: FileType = FileType {
    file_format: &FileFormat {
        id: 48_005_022,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Database, version 97",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
