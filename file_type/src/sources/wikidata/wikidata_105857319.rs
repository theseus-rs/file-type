use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857319: FileFormat = FileFormat {
    id: 105_857_319,
    puid: "wikidata/105857319",
    name: "Show Partner Animator Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x68, 0x6F, 0x77, 0x20, 0x50, 0x61, 0x72, 0x74, 0x6E, 0x65, 0x72, 0x20,
                    0x41, 0x6E, 0x69, 0x6D, 0x61, 0x74, 0x6F, 0x72, 0x20, 0x48, 0x65, 0x6C, 0x70,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
