use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333324: FileFormat = FileFormat {
    id: 111_333_324,
    source_type: SourceType::Wikidata,
    name: "Turtle Beach Pinnacle sound bank",
    extensions: &["psb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
