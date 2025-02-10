use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59537246: FileType = FileType {
    file_format: &FileFormat {
        id: 59_537_246,
        source_type: SourceType::Wikidata,
        name: "Microsoft OneNote file format",
        extensions: &["one"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
