use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_6128185: FileType = FileType {
    file_format: &FileFormat {
        id: 6_128_185,
        source_type: SourceType::Wikidata,
        name: "SigmaTel Motion Video",
        extensions: &["smv"],
        media_types: &["video/x-smv"],
        signatures: &[],
        related_formats: &[],
    },
};
