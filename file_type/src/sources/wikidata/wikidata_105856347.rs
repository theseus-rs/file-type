use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856347: FileFormat = FileFormat {
    id: 105_856_347,
    source_type: SourceType::Wikidata,
    name: "DeskMate Paint image",
    extensions: &["pnt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x13, 0x50, 0x4E, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
