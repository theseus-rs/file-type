use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858766: FileFormat = FileFormat {
    id: 105_858_766,
    source_type: SourceType::Wikidata,
    name: "Brother Embroidery File",
    extensions: &["bdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x4D, 0x50, 0x42, 0x4D, 0x48, 0x4C, 0x4D, 0x4B, 0x59, 0x49, 0x44, 0x48,
                    0x53, 0x46, 0x48, 0x44, 0x4D, 0x42, 0x4A, 0x50, 0x41, 0x4C, 0x4A, 0x45, 0x52,
                    0x42, 0x4A, 0x54, 0x59, 0x4D, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
