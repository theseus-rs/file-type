use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119786627: FileType = FileType {
    file_format: &FileFormat {
        id: 119_786_627,
        source_type: SourceType::Wikidata,
        name: "Export v4 File",
        extensions: &["mxp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
