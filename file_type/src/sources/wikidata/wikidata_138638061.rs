use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138638061: FileType = FileType {
    file_format: &FileFormat {
        id: 138_638_061,
        source_type: SourceType::Wikidata,
        name: "DC2-NPGS file format",
        extensions: &["dc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
