use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127703853: FileType = FileType {
    file_format: &FileFormat {
        id: 127_703_853,
        source_type: SourceType::Wikidata,
        name: "Idris source code file",
        extensions: &["idr"],
        media_types: &["text/x-idris"],
        signatures: &[],
        related_formats: &[],
    },
};
