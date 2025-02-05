use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859954: FileFormat = FileFormat {
    id: 105_859_954,
    source_type: SourceType::Wikidata,
    name: "trsvid TV6 video",
    extensions: &["tv6"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x57, 0x50, 0xD6, 0x06])],
            },
        }],
    }],
    related_formats: &[],
};
