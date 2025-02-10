use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856933: FileFormat = FileFormat {
    id: 105_856_933,
    source_type: SourceType::Wikidata,
    name: "NETGEN Constructive Solid Geometry format (with rem)",
    extensions: &["geo"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23])],
            },
        }],
    }],
    related_formats: &[],
};
