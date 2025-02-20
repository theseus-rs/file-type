use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122148070: FileType = FileType {
    file_format: &FileFormat {
        id: 122_148_070,
        source_type: SourceType::Wikidata,
        name: "Finale Performance Assessment",
        extensions: &["fpa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
