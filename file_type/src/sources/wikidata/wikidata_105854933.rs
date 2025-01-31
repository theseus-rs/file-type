use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854933: FileFormat = FileFormat {
    id: 105_854_933,
    puid: "wikidata/105854933",
    name: "BlackBerry Application Loader",
    extensions: &["alx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6C, 0x6F, 0x61, 0x64, 0x65, 0x72, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                    0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
