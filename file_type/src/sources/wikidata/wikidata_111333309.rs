use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333309: FileFormat = FileFormat {
    id: 111_333_309,
    source_type: SourceType::Wikidata,
    name: "Turtle Beach Pinnacle program file",
    extensions: &["ppf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
