use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116790608: FileType = FileType {
    file_format: &FileFormat {
        id: 116_790_608,
        source_type: SourceType::Wikidata,
        name: "InDesign template",
        extensions: &["indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
