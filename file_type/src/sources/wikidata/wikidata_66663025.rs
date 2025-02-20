use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66663025: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_025,
        source_type: SourceType::Wikidata,
        name: "Lotus Freelance SmartMaster Content",
        extensions: &["smc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
