use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858135: FileFormat = FileFormat {
    id: 105_858_135,
    puid: "wikidata/105858135",
    name: "uBee512 KCS tape image",
    extensions: &["tap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x41, 0x50, 0x5F, 0x4B, 0x43, 0x53, 0x5F, 0x4D, 0x42, 0x45, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
