use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116941363: FileType = FileType {
    file_format: &FileFormat {
        id: 116_941_363,
        source_type: SourceType::Wikidata,
        name: "Print Perfect Document",
        extensions: &["pub"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
