use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356213: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_213,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif (6/7/8) 'waves' format",
        extensions: &["w2w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
