use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131062662: FileType = FileType {
    file_format: &FileFormat {
        id: 131_062_662,
        source_type: SourceType::Wikidata,
        name: "SNOBOL4 file format",
        extensions: &["snobol"],
        media_types: &["text/x-snobol"],
        signatures: &[],
        related_formats: &[],
    },
};
