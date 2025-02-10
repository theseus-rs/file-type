use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27866075: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_075,
        source_type: SourceType::Wikidata,
        name: "ABC notation, version 1.6",
        extensions: &["abc", "abh"],
        media_types: &["text/vnd.abc"],
        signatures: &[],
        related_formats: &[],
    },
};
