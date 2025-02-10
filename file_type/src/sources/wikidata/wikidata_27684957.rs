use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27684957: FileType = FileType {
    file_format: &FileFormat {
        id: 27_684_957,
        source_type: SourceType::Wikidata,
        name: "Microsoft Publisher Pack and Go file format",
        extensions: &["puz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
