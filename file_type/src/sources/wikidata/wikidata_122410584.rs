use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122410584: FileType = FileType {
    file_format: &FileFormat {
        id: 122_410_584,
        source_type: SourceType::Wikidata,
        name: "PowerPC Symbol File",
        extensions: &["xsym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
