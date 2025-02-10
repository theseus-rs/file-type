use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109673241: FileType = FileType {
    file_format: &FileFormat {
        id: 109_673_241,
        source_type: SourceType::Wikidata,
        name: "MS Outlook Express Email Index",
        extensions: &["idx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
