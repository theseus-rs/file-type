use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105047785: FileType = FileType {
    file_format: &FileFormat {
        id: 105_047_785,
        source_type: SourceType::Wikidata,
        name: "Binary Color Format",
        extensions: &["bcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
