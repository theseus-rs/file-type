use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857867: FileFormat = FileFormat {
    id: 105_857_867,
    source_type: SourceType::Wikidata,
    name: "Floppy Diskette Copy disk image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x6C, 0x6F, 0x70, 0x70, 0x79, 0x20, 0x44, 0x69, 0x73, 0x6B, 0x65, 0x74,
                    0x74, 0x65, 0x20, 0x43, 0x6F, 0x70, 0x79, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65,
                    0x2E, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
