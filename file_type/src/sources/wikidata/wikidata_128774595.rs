use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128774595: FileType = FileType {
    file_format: &FileFormat {
        id: 128_774_595,
        source_type: SourceType::Wikidata,
        name: "Common Lisp file format",
        extensions: &["cl"],
        media_types: &["text/x-common-lisp"],
        signatures: &[],
        related_formats: &[],
    },
};
