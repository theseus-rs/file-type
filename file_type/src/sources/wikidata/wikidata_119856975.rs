use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119856975: FileType = FileType {
    file_format: &FileFormat {
        id: 119_856_975,
        source_type: SourceType::Wikidata,
        name: "Streets & Trips File",
        extensions: &["est"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
