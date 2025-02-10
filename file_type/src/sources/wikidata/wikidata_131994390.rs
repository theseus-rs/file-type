use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131994390: FileType = FileType {
    file_format: &FileFormat {
        id: 131_994_390,
        source_type: SourceType::Wikidata,
        name: "Web Extracted Text",
        extensions: &["wet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
