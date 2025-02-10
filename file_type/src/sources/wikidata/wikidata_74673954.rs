use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_74673954: FileType = FileType {
    file_format: &FileFormat {
        id: 74_673_954,
        source_type: SourceType::Wikidata,
        name: "TurboTax return file",
        extensions: &["tax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
