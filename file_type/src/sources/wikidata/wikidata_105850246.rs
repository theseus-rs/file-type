use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850246: FileFormat = FileFormat {
    id: 105_850_246,
    puid: "wikidata/105850246",
    name: "ControllerMate programming",
    extensions: &["cmate"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x62, 0x70, 0x6C, 0x69, 0x73, 0x74, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
