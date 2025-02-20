use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28791369: FileType = FileType {
    file_format: &FileFormat {
        id: 28_791_369,
        source_type: SourceType::Wikidata,
        name: "App Installer package",
        extensions: &["appx", "appxbundle"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
