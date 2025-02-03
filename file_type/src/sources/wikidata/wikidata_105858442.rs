use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858442: FileFormat = FileFormat {
    id: 105_858_442,
    source_type: SourceType::Wikidata,
    name: "Pocket Tanks emitter",
    extensions: &["emi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x51, 0x4E, 0x51, 0x57, 0x15, 0x09, 0x4A, 0x3F, 0x5D, 0x50, 0x4C, 0x3D,
                    0x4C, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
