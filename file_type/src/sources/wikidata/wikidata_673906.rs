use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_673906: FileType = FileType {
    file_format: &FileFormat {
        id: 673_906,
        source_type: SourceType::Wikidata,
        name: "Simple Standards-Based Slide Show System",
        extensions: &["html", "xhtml"],
        media_types: &["application/xhtml+xml", "text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
