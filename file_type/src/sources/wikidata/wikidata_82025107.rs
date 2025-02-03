use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_82025107: FileFormat = FileFormat {
    id: 82_025_107,
    source_type: SourceType::Wikidata,
    name: "Electronic Book Exchange",
    extensions: &["ebx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x53, 0x74, 0x61, 0x72, 0x74, 0x20, 0x65, 0x42, 0x4F, 0x4F, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
