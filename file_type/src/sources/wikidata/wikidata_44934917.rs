use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_44934917: FileType = FileType {
    file_format: &FileFormat {
        id: 44_934_917,
        source_type: SourceType::Wikidata,
        name: "Macintosh Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
