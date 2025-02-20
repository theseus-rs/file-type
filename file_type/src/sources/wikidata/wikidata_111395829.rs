use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111395829: FileType = FileType {
    file_format: &FileFormat {
        id: 111_395_829,
        source_type: SourceType::Wikidata,
        name: "PhotoSuite Project File",
        extensions: &["pzp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
