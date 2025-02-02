use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853187: FileFormat = FileFormat {
    id: 105_853_187,
    source_type: SourceType::Wikidata,
    name: "Sound Forge Peak Data File",
    extensions: &["sfk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x46, 0x50, 0x4B, 0x01, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
