use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131417573: FileType = FileType {
    file_format: &FileFormat {
        id: 131_417_573,
        source_type: SourceType::Wikidata,
        name: "FRACT file",
        extensions: &["FRACT"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
