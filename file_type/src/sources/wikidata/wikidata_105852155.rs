use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852155: FileFormat = FileFormat {
    id: 105_852_155,
    puid: "wikidata/105852155",
    name: "Windows Shadow spooler (2000/XP)",
    extensions: &["shd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x67, 0x49, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
