use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4928413: FileType = FileType {
    file_format: &FileFormat {
        id: 4_928_413,
        source_type: SourceType::Wikidata,
        name: "Blorb",
        extensions: &["blb", "blorb", "gblorb", "glb", "zblorb", "zlb"],
        media_types: &["application/x-blorb"],
        signatures: &[],
        related_formats: &[],
    },
};
