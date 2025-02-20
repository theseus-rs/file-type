use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51839187: FileType = FileType {
    file_format: &FileFormat {
        id: 51_839_187,
        source_type: SourceType::Wikidata,
        name: "NAP Metafile",
        extensions: &["nap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
