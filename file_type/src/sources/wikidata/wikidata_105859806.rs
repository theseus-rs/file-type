use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859806: FileFormat = FileFormat {
    id: 105_859_806,
    source_type: SourceType::Wikidata,
    name: "ViX images catalog",
    extensions: &["vix"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x69, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
