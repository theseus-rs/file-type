use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113864136: FileType = FileType {
    file_format: &FileFormat {
        id: 113_864_136,
        source_type: SourceType::Wikidata,
        name: "UDF Disc Image",
        extensions: &["cdr", "dmg", "iso", "toast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
