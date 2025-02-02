use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111722157: FileFormat = FileFormat {
    id: 111_722_157,
    source_type: SourceType::Wikidata,
    name: "WiDE Project File",
    extensions: &["wpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
