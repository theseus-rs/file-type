use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866076: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_076,
        source_type: SourceType::Wikidata,
        name: "ABC notation, version 2.0",
        extensions: &["abc", "abh"],
        media_types: &["text/vnd.abc"],
        signatures: &[],
        related_formats: &[],
    },
};
