use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122229335: FileType = FileType {
    file_format: &FileFormat {
        id: 122_229_335,
        source_type: SourceType::Wikidata,
        name: "WPA-PSK Export Hash",
        extensions: &["wph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
