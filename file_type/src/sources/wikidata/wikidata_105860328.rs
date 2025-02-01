use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860328: FileFormat = FileFormat {
    id: 105_860_328,
    puid: "wikidata/105860328",
    name: "WinRAR interface theme",
    extensions: &["rar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x61, 0x72, 0x21, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
