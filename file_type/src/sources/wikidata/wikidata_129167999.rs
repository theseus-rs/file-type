use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129167999: FileType = FileType {
    file_format: &FileFormat {
        id: 129_167_999,
        source_type: SourceType::Wikidata,
        name: "Factor source code file",
        extensions: &["factor"],
        media_types: &["text/x-factor"],
        signatures: &[],
        related_formats: &[],
    },
};
