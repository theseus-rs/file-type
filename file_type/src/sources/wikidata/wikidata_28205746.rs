use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205746: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_746,
        source_type: SourceType::Wikidata,
        name: "Microsoft Fax At Work Document",
        extensions: &["awd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
