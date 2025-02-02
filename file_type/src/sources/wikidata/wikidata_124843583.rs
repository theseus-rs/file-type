use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124843583: FileFormat = FileFormat {
    id: 124_843_583,
    source_type: SourceType::Wikidata,
    name: "XTiger template",
    extensions: &["xtd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
