use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111342726: FileType = FileType {
    file_format: &FileFormat {
        id: 111_342_726,
        source_type: SourceType::Wikidata,
        name: "SPPack sound sample",
        extensions: &["sppack"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
