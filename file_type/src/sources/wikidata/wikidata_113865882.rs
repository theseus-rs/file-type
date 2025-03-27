use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113865882: FileType = FileType {
    file_format: &FileFormat {
        id: 113_865_882,
        source_type: SourceType::Wikidata,
        name: "UDF-ISO 9660 Bridge Disc",
        extensions: &["cdr", "dmg", "iso", "toast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
