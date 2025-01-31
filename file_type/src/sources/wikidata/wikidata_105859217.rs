use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859217: FileFormat = FileFormat {
    id: 105_859_217,
    puid: "wikidata/105859217",
    name: "Besiege machine",
    extensions: &["bsg"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x52, 0x45, 0x46, 0x41, 0x42, 0x20, 0x49, 0x44, 0x53, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
