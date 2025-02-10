use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47012425: FileType = FileType {
    file_format: &FileFormat {
        id: 47_012_425,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro Table file format",
        extensions: &["dbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
