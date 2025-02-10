use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63391705: FileType = FileType {
    file_format: &FileFormat {
        id: 63_391_705,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for DOS, version 3b",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
