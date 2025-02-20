use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132348514: FileType = FileType {
    file_format: &FileFormat {
        id: 132_348_514,
        source_type: SourceType::Wikidata,
        name: "React TypeScript File format",
        extensions: &["tsx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
