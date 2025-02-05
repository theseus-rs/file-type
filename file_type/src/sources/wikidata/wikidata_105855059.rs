use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855059: FileFormat = FileFormat {
    id: 105_855_059,
    source_type: SourceType::Wikidata,
    name: "JARCS compressed archive",
    extensions: &["jar"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x41, 0x52, 0x43, 0x53, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
