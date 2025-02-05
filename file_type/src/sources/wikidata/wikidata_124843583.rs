use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124843583: FileFormat = FileFormat {
    id: 124_843_583,
    source_type: SourceType::Wikidata,
    name: "XTiger template",
    extensions: &["xtd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
