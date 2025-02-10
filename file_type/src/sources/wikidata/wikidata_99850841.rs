use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99850841: FileType = FileType {
    file_format: &FileFormat {
        id: 99_850_841,
        source_type: SourceType::Wikidata,
        name: "Picture Publisher Bitmap 6-10",
        extensions: &["ppf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
