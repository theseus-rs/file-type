use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51333820: FileType = FileType {
    file_format: &FileFormat {
        id: 51_333_820,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint Presentation Show",
        extensions: &["pps"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
