use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867579: FileFormat = FileFormat {
    id: 105_867_579,
    puid: "wikidata/105867579",
    name: "NUnit project",
    extensions: &["nunit"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x4E, 0x55, 0x6E, 0x69, 0x74, 0x50, 0x72, 0x6F, 0x6A,
                    0x65, 0x63, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
