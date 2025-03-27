use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_856364: FileType = FileType {
    file_format: &FileFormat {
        id: 856_364,
        source_type: SourceType::Wikidata,
        name: "Common Object File Format",
        extensions: &["o", "obj"],
        media_types: &["application/x-coff", "application/x-coffexec"],
        signatures: &[],
        related_formats: &[],
    },
};
