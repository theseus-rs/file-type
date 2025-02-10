use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975658: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_658,
        source_type: SourceType::Wikidata,
        name: "SketchUp skp",
        extensions: &["skb", "skp"],
        media_types: &["application/vnd.sketchup.skp"],
        signatures: &[],
        related_formats: &[],
    },
};
