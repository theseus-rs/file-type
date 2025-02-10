use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_43976633: FileType = FileType {
    file_format: &FileFormat {
        id: 43_976_633,
        source_type: SourceType::Wikidata,
        name: "Exchangeable Image File Format (Audio)",
        extensions: &["wav"],
        media_types: &["audio/x-wav"],
        signatures: &[],
        related_formats: &[],
    },
};
