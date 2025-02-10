use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127126460: FileType = FileType {
    file_format: &FileFormat {
        id: 127_126_460,
        source_type: SourceType::Wikidata,
        name: "Harwell-Boeing file",
        extensions: &["hb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
