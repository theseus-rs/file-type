use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860312: FileFormat = FileFormat {
    id: 105_860_312,
    source_type: SourceType::Wikidata,
    name: "ArtCraft Resource",
    extensions: &["res"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x43, 0x29, 0x20, 0x41, 0x72, 0x74, 0x43, 0x72, 0x61, 0x66, 0x74, 0x20,
                    0x52, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
