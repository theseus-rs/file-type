use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_65990735: FileType = FileType {
    file_format: &FileFormat {
        id: 65_990_735,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere Library",
        extensions: &["plb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
