use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116784703: FileType = FileType {
    file_format: &FileFormat {
        id: 116_784_703,
        source_type: SourceType::Wikidata,
        name: "Form Designer Pro Form Contents",
        extensions: &["ofx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
