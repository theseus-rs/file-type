use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129085220: FileType = FileType {
    file_format: &FileFormat {
        id: 129_085_220,
        source_type: SourceType::Wikidata,
        name: "elpi file format",
        extensions: &["elpi"],
        media_types: &["text/x-elpi"],
        signatures: &[],
        related_formats: &[],
    },
};
