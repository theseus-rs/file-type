use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967083: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_083,
        source_type: SourceType::Wikidata,
        name: "Digital Illusions",
        extensions: &["di"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
