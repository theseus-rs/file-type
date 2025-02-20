use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67173026: FileType = FileType {
    file_format: &FileFormat {
        id: 67_173_026,
        source_type: SourceType::Wikidata,
        name: "GIMP compressed XJT Image",
        extensions: &["xjt", "xjtbz", "xjtgz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
