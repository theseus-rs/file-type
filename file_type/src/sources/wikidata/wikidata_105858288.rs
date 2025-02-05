use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858288: FileFormat = FileFormat {
    id: 105_858_288,
    source_type: SourceType::Wikidata,
    name: "MetaQuotes Language 4 compiled program",
    extensions: &["ex4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x58, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
