use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_86451671: FileType = FileType {
    file_format: &FileFormat {
        id: 86_451_671,
        source_type: SourceType::Wikidata,
        name: "RFFlow Chart 4",
        extensions: &["flo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
