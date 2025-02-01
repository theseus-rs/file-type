use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852160: FileFormat = FileFormat {
    id: 105_852_160,
    puid: "wikidata/105852160",
    name: "Scalable Vector Graphics (var.2)",
    extensions: &["svg"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x73, 0x76, 0x67,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
