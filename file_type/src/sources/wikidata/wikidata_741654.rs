use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_741654: FileFormat = FileFormat {
    id: 741_654,
    puid: "wikidata/741654",
    name: "DirectDraw Surface",
    extensions: &["dds"],
    media_types: &["image/vnd-ms.dds"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x44, 0x53, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
