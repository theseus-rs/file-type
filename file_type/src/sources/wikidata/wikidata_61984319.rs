use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61984319: FileType = FileType {
    file_format: &FileFormat {
        id: 61_984_319,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro Class Library",
        extensions: &["vcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
