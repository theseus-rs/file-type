use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127805343: FileType = FileType {
    file_format: &FileFormat {
        id: 127_805_343,
        source_type: SourceType::Wikidata,
        name: "njs script file",
        extensions: &["njs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
