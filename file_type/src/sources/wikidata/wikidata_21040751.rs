use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_21040751: FileType = FileType {
    file_format: &FileFormat {
        id: 21_040_751,
        source_type: SourceType::Wikidata,
        name: "Farandole Composer format",
        extensions: &["far"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
