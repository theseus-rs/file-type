use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_43866871: FileType = FileType {
    file_format: &FileFormat {
        id: 43_866_871,
        source_type: SourceType::Wikidata,
        name: "PCX, version 2",
        extensions: &["pcc", "pcx"],
        media_types: &["image/vnd.zbrush.pcx", "image/x-pcx"],
        signatures: &[],
        related_formats: &[],
    },
};
