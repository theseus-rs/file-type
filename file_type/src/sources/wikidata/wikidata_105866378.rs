use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866378: FileFormat = FileFormat {
    id: 105_866_378,
    puid: "wikidata/105866378",
    name: "Post-It Software Note Template",
    extensions: &["psntemplate"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6E, 0x6F, 0x74, 0x65, 0x3E, 0x3C, 0x63, 0x6C, 0x73, 0x69, 0x64, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
