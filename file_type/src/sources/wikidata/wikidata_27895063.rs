use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27895063: FileFormat = FileFormat {
    id: 27_895_063,
    source_type: SourceType::Wikidata,
    name: "Windows Media Video",
    extensions: &["wm", "wmv"],
    media_types: &["video/x-ms-wmv"],
    internal_signatures: &[],
    related_formats: &[],
};
