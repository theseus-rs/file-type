use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858359: FileFormat = FileFormat {
    id: 105_858_359,
    source_type: SourceType::Wikidata,
    name: "E-Studio 1.x experiment",
    extensions: &["es"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
