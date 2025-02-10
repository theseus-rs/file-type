use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_86451849: FileType = FileType {
    file_format: &FileFormat {
        id: 86_451_849,
        source_type: SourceType::Wikidata,
        name: "RFFlow Chart 5",
        extensions: &["flo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
