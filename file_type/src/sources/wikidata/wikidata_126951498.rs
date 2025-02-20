use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126951498: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_498,
        source_type: SourceType::Wikidata,
        name: "Haxe source code file",
        extensions: &["hx"],
        media_types: &["text/haxe", "text/x-haxe", "text/x-hx"],
        signatures: &[],
        related_formats: &[],
    },
};
