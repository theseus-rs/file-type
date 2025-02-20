use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167436: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_436,
        source_type: SourceType::Wikidata,
        name: "Microsoft Object Description Language",
        extensions: &["odl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
