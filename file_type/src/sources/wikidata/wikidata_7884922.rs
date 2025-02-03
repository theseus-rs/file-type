use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7884922: FileFormat = FileFormat {
    id: 7_884_922,
    source_type: SourceType::Wikidata,
    name: "Unified Emulator Format",
    extensions: &["uef"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x45, 0x46, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x21, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
