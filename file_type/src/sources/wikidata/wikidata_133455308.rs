use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133455308: FileType = FileType {
    file_format: &FileFormat {
        id: 133_455_308,
        source_type: SourceType::Wikidata,
        name: "AMOS Picture Packer",
        extensions: &["bin"],
        media_types: &["image/x-amos-picturepacker"],
        signatures: &[],
        related_formats: &[],
    },
};
