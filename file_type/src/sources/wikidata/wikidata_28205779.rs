use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205779: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_779,
        source_type: SourceType::Wikidata,
        name: "Bob ray tracer bitmap",
        extensions: &["bob"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
