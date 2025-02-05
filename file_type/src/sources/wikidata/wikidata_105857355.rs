use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857355: FileFormat = FileFormat {
    id: 105_857_355,
    source_type: SourceType::Wikidata,
    name: "Fishing Simulator 2 addon",
    extensions: &["jr2"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x52, 0x32, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
