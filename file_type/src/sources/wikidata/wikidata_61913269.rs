use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61913269: FileType = FileType {
    file_format: &FileFormat {
        id: 61_913_269,
        source_type: SourceType::Wikidata,
        name: "Microsoft Project Export File, version 4",
        extensions: &["mpx"],
        media_types: &["application/x-project"],
        signatures: &[],
        related_formats: &[],
    },
};
