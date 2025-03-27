use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_629039: FileType = FileType {
    file_format: &FileFormat {
        id: 629_039,
        source_type: SourceType::Wikidata,
        name: "JBIG",
        extensions: &["bie", "jbg", "jbig"],
        media_types: &["image/jbig"],
        signatures: &[],
        related_formats: &[],
    },
};
