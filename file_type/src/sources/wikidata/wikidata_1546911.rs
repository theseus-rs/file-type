use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1546911: FileType = FileType {
    file_format: &FileFormat {
        id: 1_546_911,
        source_type: SourceType::Wikidata,
        name: "Cross-Platform Installer Module",
        extensions: &["xpi"],
        media_types: &["application/x-xpinstall"],
        signatures: &[],
        related_formats: &[],
    },
};
