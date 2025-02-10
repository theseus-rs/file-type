use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131453299: FileType = FileType {
    file_format: &FileFormat {
        id: 131_453_299,
        source_type: SourceType::Wikidata,
        name: "YARA file format",
        extensions: &["yar"],
        media_types: &["text/x-yara"],
        signatures: &[],
        related_formats: &[],
    },
};
