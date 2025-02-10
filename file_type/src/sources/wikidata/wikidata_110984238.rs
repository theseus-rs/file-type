use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110984238: FileType = FileType {
    file_format: &FileFormat {
        id: 110_984_238,
        source_type: SourceType::Wikidata,
        name: "Ulead Image Sequence",
        extensions: &["uis"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
