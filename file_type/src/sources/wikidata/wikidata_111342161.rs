use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111342161: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_161,
        source_type: SourceType::Wikidata,
        name: "Ad Lib Gold sample",
        extensions: &["smp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
