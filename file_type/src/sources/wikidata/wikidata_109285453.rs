use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109285453: FileType = FileType {
    file_format: &FileFormat {
        id: 109_285_453,
        source_type: SourceType::Wikidata,
        name: "phtml",
        extensions: &["phtml"],
        media_types: &["application/x-httpd-php"],
        signatures: &[],
        related_formats: &[],
    },
};
