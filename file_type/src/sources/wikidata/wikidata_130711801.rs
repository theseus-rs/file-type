use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130711801: FileType = FileType {
    file_format: &FileFormat {
        id: 130_711_801,
        source_type: SourceType::Wikidata,
        name: "RPMSpec file format",
        extensions: &["spec"],
        media_types: &["text/x-rpm-spec"],
        signatures: &[],
        related_formats: &[],
    },
};
