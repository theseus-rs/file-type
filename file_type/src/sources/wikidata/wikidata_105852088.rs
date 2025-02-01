use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852088: FileFormat = FileFormat {
    id: 105_852_088,
    puid: "wikidata/105852088",
    name: "MegaZeux Save (v1.0x)",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x5A, 0x53, 0x41, 0x56, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
