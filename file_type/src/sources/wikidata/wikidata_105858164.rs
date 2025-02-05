use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858164: FileFormat = FileFormat {
    id: 105_858_164,
    source_type: SourceType::Wikidata,
    name: "Sound Blaster Instrument Bank",
    extensions: &["ibk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x42, 0x4B, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
