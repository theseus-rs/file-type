use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131626588: FileType = FileType {
    file_format: &FileFormat {
        id: 131_626_588,
        source_type: SourceType::Wikidata,
        name: "FLUENT CFF file format",
        extensions: &["dat.h5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
