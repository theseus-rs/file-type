use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855008: FileFormat = FileFormat {
    id: 105_855_008,
    source_type: SourceType::Wikidata,
    name: "Bag archive",
    extensions: &["bag"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x47, 0x31, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
