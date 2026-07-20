use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138698055: FileType = FileType {
    file_format: &FileFormat {
        id: 138_698_055,
        source_type: SourceType::Wikidata,
        name: "Vimeo DASH JSON",
        extensions: &[],
        media_types: &["application/vnd.vimeo.dash+json"],
        signatures: &[],
        related_formats: &[],
    },
};
