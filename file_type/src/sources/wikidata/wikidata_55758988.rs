use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55758988: FileType = FileType {
    file_format: &FileFormat {
        id: 55_758_988,
        source_type: SourceType::Wikidata,
        name: "Microsoft Program Database file format, version 2",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
