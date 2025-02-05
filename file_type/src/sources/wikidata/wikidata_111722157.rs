use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111722157: FileFormat = FileFormat {
    id: 111_722_157,
    source_type: SourceType::Wikidata,
    name: "WiDE Project File",
    extensions: &["wpj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
