use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50419827: FileType = FileType {
    file_format: &FileFormat {
        id: 50_419_827,
        source_type: SourceType::Wikidata,
        name: "Microsoft PRX File",
        extensions: &["prx"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
