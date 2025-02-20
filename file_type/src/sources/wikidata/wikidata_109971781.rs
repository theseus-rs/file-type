use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109971781: FileType = FileType {
    file_format: &FileFormat {
        id: 109_971_781,
        source_type: SourceType::Wikidata,
        name: "PDF Portfolio file format",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
