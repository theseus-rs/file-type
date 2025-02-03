use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111333324: FileFormat = FileFormat {
    id: 111_333_324,
    source_type: SourceType::Wikidata,
    name: "Turtle Beach Pinnacle sound bank",
    extensions: &["psb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
