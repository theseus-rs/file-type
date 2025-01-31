use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858122: FileFormat = FileFormat {
    id: 105_858_122,
    puid: "wikidata/105858122",
    name: "QDOS QL floppy disk image (DS/HD)",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x51, 0x4C, 0x35, 0x42])],
            },
        }],
    }],
    related_formats: &[],
};
