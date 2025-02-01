use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867374: FileFormat = FileFormat {
    id: 105_867_374,
    puid: "wikidata/105867374",
    name: "Native Instruments FM8 patch",
    extensions: &["nfm8"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x68, 0x73, 0x69,
                    0x6E, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
