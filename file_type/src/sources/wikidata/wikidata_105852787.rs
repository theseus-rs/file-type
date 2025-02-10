use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852787: FileFormat = FileFormat {
    id: 105_852_787,
    source_type: SourceType::Wikidata,
    name: "IEEE DASC Standard Delay Format",
    extensions: &["sdf", "sdo"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x28, 0x44, 0x45, 0x4C, 0x41, 0x59, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
