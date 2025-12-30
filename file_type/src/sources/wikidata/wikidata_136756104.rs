use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136756104: FileType = FileType {
    file_format: &FileFormat {
        id: 136_756_104,
        source_type: SourceType::Wikidata,
        name: "VirtualDJ file",
        extensions: &["vdj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
