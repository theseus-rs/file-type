use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137735925: FileType = FileType {
    file_format: &FileFormat {
        id: 137_735_925,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate preferences file",
        extensions: &["cpr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
