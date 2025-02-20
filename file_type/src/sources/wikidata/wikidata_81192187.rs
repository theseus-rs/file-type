use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_81192187: FileType = FileType {
    file_format: &FileFormat {
        id: 81_192_187,
        source_type: SourceType::Wikidata,
        name: "Microsoft Border art",
        extensions: &["bdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
