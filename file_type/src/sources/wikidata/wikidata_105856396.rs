use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856396: FileFormat = FileFormat {
    id: 105_856_396,
    source_type: SourceType::Wikidata,
    name: "WindowBlinds Progress Anim theme",
    extensions: &["wba"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04, 0x14])],
            },
        }],
    }],
    related_formats: &[],
};
