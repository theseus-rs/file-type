use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6934775: FileType = FileType {
    file_format: &FileFormat {
        id: 6_934_775,
        source_type: SourceType::Wikidata,
        name: "Multimedia Container Format",
        extensions: &["audio.mcf", "av.mcf", "mcf", "video.mcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
