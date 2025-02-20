use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117886050: FileType = FileType {
    file_format: &FileFormat {
        id: 117_886_050,
        source_type: SourceType::Wikidata,
        name: "BZip3",
        extensions: &["bz3"],
        media_types: &["application/x-bzip3"],
        signatures: &[],
        related_formats: &[],
    },
};
