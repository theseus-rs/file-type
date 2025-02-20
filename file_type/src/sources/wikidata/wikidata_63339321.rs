use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_63339321: FileType = FileType {
    file_format: &FileFormat {
        id: 63_339_321,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Database for Windows, version 4.5a",
        extensions: &["wdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
