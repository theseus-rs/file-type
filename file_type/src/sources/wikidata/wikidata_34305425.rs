use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34305425: FileType = FileType {
    file_format: &FileFormat {
        id: 34_305_425,
        source_type: SourceType::Wikidata,
        name: "Scheme script",
        extensions: &["sch", "scm", "ss"],
        media_types: &["application/x-scheme", "text/x-scheme"],
        signatures: &[],
        related_formats: &[],
    },
};
