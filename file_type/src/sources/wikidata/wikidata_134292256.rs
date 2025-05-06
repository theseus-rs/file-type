use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134292256: FileType = FileType {
    file_format: &FileFormat {
        id: 134_292_256,
        source_type: SourceType::Wikidata,
        name: "Clipper link file",
        extensions: &["lnk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
