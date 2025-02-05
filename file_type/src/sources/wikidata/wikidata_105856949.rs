use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856949: FileFormat = FileFormat {
    id: 105_856_949,
    source_type: SourceType::Wikidata,
    name: "Surfer Grid",
    extensions: &["grd"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x41, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
