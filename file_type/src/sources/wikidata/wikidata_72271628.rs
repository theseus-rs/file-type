use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_72271628: FileType = FileType {
    file_format: &FileFormat {
        id: 72_271_628,
        source_type: SourceType::Wikidata,
        name: "ndr",
        extensions: &["ndr"],
        media_types: &["unknown"],
        signatures: &[],
        related_formats: &[],
    },
};
