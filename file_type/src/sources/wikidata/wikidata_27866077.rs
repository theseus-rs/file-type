use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866077: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_077,
        source_type: SourceType::Wikidata,
        name: "ABC notation, version 2.1",
        extensions: &["abc", "abh"],
        media_types: &["text/vnd.abc"],
        signatures: &[],
        related_formats: &[],
    },
};
