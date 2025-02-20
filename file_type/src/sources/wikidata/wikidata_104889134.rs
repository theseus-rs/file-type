use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_104889134: FileType = FileType {
    file_format: &FileFormat {
        id: 104_889_134,
        source_type: SourceType::Wikidata,
        name: "Propellerhead Reason Project File",
        extensions: &["reason", "rns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
