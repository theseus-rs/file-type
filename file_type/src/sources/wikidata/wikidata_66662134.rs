use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66662134: FileType = FileType {
    file_format: &FileFormat {
        id: 66_662_134,
        source_type: SourceType::Wikidata,
        name: "Lotus Word Pro SmartMaster",
        extensions: &["mwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
