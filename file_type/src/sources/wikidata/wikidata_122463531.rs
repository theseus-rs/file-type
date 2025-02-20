use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122463531: FileType = FileType {
    file_format: &FileFormat {
        id: 122_463_531,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual Basic Include file",
        extensions: &["bi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
