use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855464: FileFormat = FileFormat {
    id: 105_855_464,
    source_type: SourceType::Wikidata,
    name: "HEC-RAS Flow file",
    extensions: &["f01", "f02", "f99"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x6C, 0x6F, 0x77, 0x20, 0x54, 0x69, 0x74, 0x6C, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
