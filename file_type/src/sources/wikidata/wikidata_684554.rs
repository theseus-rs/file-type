use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_684554: FileType = FileType {
    file_format: &FileFormat {
        id: 684_554,
        source_type: SourceType::Wikidata,
        name: "Autorun.inf",
        extensions: &[],
        media_types: &["application/x-inf", "text/inf"],
        signatures: &[],
        related_formats: &[],
    },
};
