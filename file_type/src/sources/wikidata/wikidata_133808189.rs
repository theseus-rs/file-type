use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133808189: FileType = FileType {
    file_format: &FileFormat {
        id: 133_808_189,
        source_type: SourceType::Wikidata,
        name: "MGR Systems Teletext file",
        extensions: &["tti"],
        media_types: &["text/x.teletext.tti"],
        signatures: &[],
        related_formats: &[],
    },
};
