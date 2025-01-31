use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850839: FileFormat = FileFormat {
    id: 105_850_839,
    puid: "wikidata/105850839",
    name: "KOLEKO Save state",
    extensions: &["ksv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0xB9, 0x73, 0xC3, 0x6E, 0x00, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
