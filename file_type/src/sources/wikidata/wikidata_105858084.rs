use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858084: FileFormat = FileFormat {
    id: 105_858_084,
    source_type: SourceType::Wikidata,
    name: "ideaMaker project",
    extensions: &["idea"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x44, 0x45, 0x41, 0x20, 0x2D, 0x20, 0x4D, 0x41, 0x4B, 0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
