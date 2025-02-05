use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124843606: FileFormat = FileFormat {
    id: 124_843_606,
    source_type: SourceType::Wikidata,
    name: "XTiger library",
    extensions: &["xtl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
