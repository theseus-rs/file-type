use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111263379: FileType = FileType {
    file_format: &FileFormat {
        id: 111_263_379,
        source_type: SourceType::Wikidata,
        name: "FXpansion DR-008 drumkit",
        extensions: &["dr8"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
