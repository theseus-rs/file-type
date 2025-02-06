use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60342897: FileFormat = FileFormat {
    id: 60_342_897,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint Show",
    extensions: &["ppsx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
