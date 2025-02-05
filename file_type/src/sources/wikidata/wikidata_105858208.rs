use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858208: FileFormat = FileFormat {
    id: 105_858_208,
    source_type: SourceType::Wikidata,
    name: "Private Character Editor Bitmap Font",
    extensions: &["euf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x73, 0x20, 0x45, 0x55, 0x44, 0x43, 0x20,
                    0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20, 0x42, 0x69, 0x74, 0x6D, 0x61, 0x70,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x28, 0x55, 0x6E, 0x69, 0x2D, 0x43, 0x6F,
                    0x64, 0x65, 0x29, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
