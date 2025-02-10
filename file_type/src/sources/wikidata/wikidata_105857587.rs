use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857587: FileFormat = FileFormat {
    id: 105_857_587,
    source_type: SourceType::Wikidata,
    name: "SGI volume image",
    extensions: &["img", "iso"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0B, 0xE5, 0xA9, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
