use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17073241: FileType = FileType {
    file_format: &FileFormat {
        id: 17_073_241,
        source_type: SourceType::Wikidata,
        name: "Opera Show Format",
        extensions: &["html", "xhtml"],
        media_types: &["application/xhtml+xml", "text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
