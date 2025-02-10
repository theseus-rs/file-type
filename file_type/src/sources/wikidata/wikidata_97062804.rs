use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_97062804: FileType = FileType {
    file_format: &FileFormat {
        id: 97_062_804,
        source_type: SourceType::Wikidata,
        name: "X-Motif UIL table",
        extensions: &["uil"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
