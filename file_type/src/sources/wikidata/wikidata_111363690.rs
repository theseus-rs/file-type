use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111363690: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_690,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif XS 'waves' format",
        extensions: &["x0w"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
