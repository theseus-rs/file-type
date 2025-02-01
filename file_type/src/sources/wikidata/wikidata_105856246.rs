use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856246: FileFormat = FileFormat {
    id: 105_856_246,
    puid: "wikidata/105856246",
    name: "DoomEd ASCII map",
    extensions: &["dwd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x6F, 0x72, 0x6C, 0x64, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x76,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
