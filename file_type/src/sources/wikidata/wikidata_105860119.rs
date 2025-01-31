use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860119: FileFormat = FileFormat {
    id: 105_860_119,
    puid: "wikidata/105860119",
    name: "Emu80 RK snapshot",
    extensions: &["rss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4B, 0x53, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
