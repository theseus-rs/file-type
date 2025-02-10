use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61984341: FileType = FileType {
    file_format: &FileFormat {
        id: 61_984_341,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro database container (memo files)",
        extensions: &["dct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
