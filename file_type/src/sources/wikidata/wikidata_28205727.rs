use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205727: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_727,
        source_type: SourceType::Wikidata,
        name: "AVS X image",
        extensions: &["avs", "mbfavs", "x"],
        media_types: &["image/x-avsx"],
        signatures: &[],
        related_formats: &[],
    },
};
