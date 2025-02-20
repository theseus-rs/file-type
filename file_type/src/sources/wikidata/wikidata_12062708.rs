use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_12062708: FileType = FileType {
    file_format: &FileFormat {
        id: 12_062_708,
        source_type: SourceType::Wikidata,
        name: "CDW file format",
        extensions: &["cdw"],
        media_types: &["image/cdw"],
        signatures: &[],
        related_formats: &[],
    },
};
