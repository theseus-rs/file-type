use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66759442: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_442,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Database Templates",
        extensions: &["accdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
