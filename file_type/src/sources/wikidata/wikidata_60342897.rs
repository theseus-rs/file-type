use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60342897: FileFormat = FileFormat {
    id: 60_342_897,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint Show",
    extensions: &["ppsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
