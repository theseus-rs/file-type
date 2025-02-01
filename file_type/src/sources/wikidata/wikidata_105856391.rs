use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856391: FileFormat = FileFormat {
    id: 105_856_391,
    puid: "wikidata/105856391",
    name: "Macromedia Director Java Resource - Video",
    extensions: &["djr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x43, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
