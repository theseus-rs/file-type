use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_8042305: FileType = FileType {
    file_format: &FileFormat {
        id: 8_042_305,
        source_type: SourceType::Wikidata,
        name: "Extensible Music Format",
        extensions: &["xmf"],
        media_types: &["audio/mobile-xmf", "audio/vnd.nokia.mobile-xmf"],
        signatures: &[],
        related_formats: &[],
    },
};
