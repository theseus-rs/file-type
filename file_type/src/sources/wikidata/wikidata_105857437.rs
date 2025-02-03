use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857437: FileFormat = FileFormat {
    id: 105_857_437,
    source_type: SourceType::Wikidata,
    name: "Lotus 123 Worksheet (V97)",
    extensions: &["123"],
    media_types: &["application/vnd.lotus-1-2-3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x1A, 0x00, 0x03, 0x10, 0x04, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
