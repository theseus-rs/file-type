use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51994105: FileType = FileType {
    file_format: &FileFormat {
        id: 51_994_105,
        source_type: SourceType::Wikidata,
        name: "IBM DisplayWrite Revisable Form Text File",
        extensions: &["rft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
