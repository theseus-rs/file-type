use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856885: FileFormat = FileFormat {
    id: 105_856_885,
    puid: "wikidata/105856885",
    name: "Galaxkey encrypted data",
    extensions: &["gxk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x47, 0x53, 0x53, 0x23, 0x03, 0x00, 0x00, 0x00, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
