use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127784231: FileType = FileType {
    file_format: &FileFormat {
        id: 127_784_231,
        source_type: SourceType::Wikidata,
        name: "Logtalk source file",
        extensions: &["lgt"],
        media_types: &["text/x-logtalk"],
        signatures: &[],
        related_formats: &[],
    },
};
