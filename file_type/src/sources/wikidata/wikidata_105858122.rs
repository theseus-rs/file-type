use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858122: FileFormat = FileFormat {
    id: 105_858_122,
    source_type: SourceType::Wikidata,
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
