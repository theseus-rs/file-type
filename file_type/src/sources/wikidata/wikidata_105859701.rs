use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859701: FileFormat = FileFormat {
    id: 105_859_701,
    puid: "wikidata/105859701",
    name: "Vis5D dataset object",
    extensions: &["v5d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x35, 0x44, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
