use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855833: FileFormat = FileFormat {
    id: 105_855_833,
    source_type: SourceType::Wikidata,
    name: "DeskMate Draw drawing",
    extensions: &["fig"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x14, 0x46, 0x49, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
