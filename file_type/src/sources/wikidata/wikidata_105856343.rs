use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856343: FileFormat = FileFormat {
    id: 105_856_343,
    source_type: SourceType::Wikidata,
    name: "PestPatrol data / scan strings",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x45, 0x53, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
