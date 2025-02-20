use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29167502: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_502,
        source_type: SourceType::Wikidata,
        name: "Open Web App Manifest",
        extensions: &["webapp"],
        media_types: &["application/x-web-app-manifest+json"],
        signatures: &[],
        related_formats: &[],
    },
};
