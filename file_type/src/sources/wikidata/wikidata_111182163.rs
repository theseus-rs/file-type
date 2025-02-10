use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111182163: FileType = FileType {
    file_format: &FileFormat {
        id: 111_182_163,
        source_type: SourceType::Wikidata,
        name: "Dreamweaver Webpage Template",
        extensions: &["dwt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
