use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121788310: FileType = FileType {
    file_format: &FileFormat {
        id: 121_788_310,
        source_type: SourceType::Wikidata,
        name: "Microsoft Powerpoint for Macintosh 3",
        extensions: &["ppt"],
        media_types: &["application/vnd.ms-PowerPoint"],
        signatures: &[],
        related_formats: &[],
    },
};
