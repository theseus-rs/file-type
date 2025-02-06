use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858537: FileFormat = FileFormat {
    id: 105_858_537,
    source_type: SourceType::Wikidata,
    name: "Houdini Binary LUT (log)",
    extensions: &["blut"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x4C, 0x55, 0x54, 0x00, 0x00, 0x00, 0x03,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
