use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854966: FileFormat = FileFormat {
    id: 105_854_966,
    source_type: SourceType::Wikidata,
    name: "RealFlight Radio Control Flying Site - Airport",
    extensions: &["airports"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x50, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
