use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125150598: FileType = FileType {
    file_format: &FileFormat {
        id: 125_150_598,
        source_type: SourceType::Wikidata,
        name: "Gliffy diagram (gxml)",
        extensions: &["gxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
