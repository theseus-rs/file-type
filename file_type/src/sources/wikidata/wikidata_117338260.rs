use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117338260: FileType = FileType {
    file_format: &FileFormat {
        id: 117_338_260,
        source_type: SourceType::Wikidata,
        name: "Corel Library",
        extensions: &["clb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
