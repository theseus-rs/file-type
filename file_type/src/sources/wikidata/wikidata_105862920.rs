use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862920: FileFormat = FileFormat {
    id: 105_862_920,
    source_type: SourceType::Wikidata,
    name: "Wolfpack Song",
    extensions: &["ms3"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x33, 0x2D, 0x4B, 0x47, 0x46, 0x27, 0x39, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
