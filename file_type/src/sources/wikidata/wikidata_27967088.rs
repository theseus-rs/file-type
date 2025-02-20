use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967088: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_088,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts MUS",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
