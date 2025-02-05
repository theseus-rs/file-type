use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859191: FileFormat = FileFormat {
    id: 105_859_191,
    source_type: SourceType::Wikidata,
    name: "Fractal Image Format bitmap (alt)",
    extensions: &["fif"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x54, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
