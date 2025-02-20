use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109673475: FileType = FileType {
    file_format: &FileFormat {
        id: 109_673_475,
        source_type: SourceType::Wikidata,
        name: "Eudora Mailbox",
        extensions: &["mbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
