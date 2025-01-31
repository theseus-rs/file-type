use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861866: FileFormat = FileFormat {
    id: 105_861_866,
    puid: "wikidata/105861866",
    name: "Freeplane MindMap",
    extensions: &["mm"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x6D, 0x61, 0x70, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
