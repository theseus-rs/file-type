use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859758: FileFormat = FileFormat {
    id: 105_859_758,
    source_type: SourceType::Wikidata,
    name: "EGG video",
    extensions: &["egg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x47, 0x47, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
