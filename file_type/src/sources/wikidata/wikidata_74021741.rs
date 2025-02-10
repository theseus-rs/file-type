use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_74021741: FileType = FileType {
    file_format: &FileFormat {
        id: 74_021_741,
        source_type: SourceType::Wikidata,
        name: "Microsoft Resource Library",
        extensions: &["rll"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
