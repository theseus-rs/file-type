use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_85712350: FileType = FileType {
    file_format: &FileFormat {
        id: 85_712_350,
        source_type: SourceType::Wikidata,
        name: "Calendar Creator File 7-8",
        extensions: &["bcc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
