use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979378: FileFormat = FileFormat {
    id: 27_979_378,
    puid: "wikidata/27979378",
    name: "VobSub index",
    extensions: &["idx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x56, 0x6F, 0x62, 0x53, 0x75, 0x62, 0x20, 0x69, 0x6E, 0x64, 0x65,
                    0x78, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x2C, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
