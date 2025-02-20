use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130054036: FileType = FileType {
    file_format: &FileFormat {
        id: 130_054_036,
        source_type: SourceType::Wikidata,
        name: "Juttle source code file",
        extensions: &["juttle"],
        media_types: &[
            "application/juttle",
            "application/x-juttle",
            "text/juttle",
            "text/x-juttle",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
