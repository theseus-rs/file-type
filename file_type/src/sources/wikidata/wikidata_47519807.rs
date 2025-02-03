use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47519807: FileFormat = FileFormat {
    id: 47_519_807,
    source_type: SourceType::Wikidata,
    name: "Serif PagePlus Publication file format, version 4",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
