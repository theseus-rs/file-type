use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122148870: FileType = FileType {
    file_format: &FileFormat {
        id: 122_148_870,
        source_type: SourceType::Wikidata,
        name: "My Logo Maker File",
        extensions: &["myf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
