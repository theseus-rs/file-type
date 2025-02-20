use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7276404: FileType = FileType {
    file_format: &FileFormat {
        id: 7_276_404,
        source_type: SourceType::Wikidata,
        name: "REX2",
        extensions: &["rex", "rx2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
