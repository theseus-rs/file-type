use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850222: FileFormat = FileFormat {
    id: 105_850_222,
    source_type: SourceType::Wikidata,
    name: "3ds UI colors",
    extensions: &["clr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
