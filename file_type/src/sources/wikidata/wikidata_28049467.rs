use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049467: FileFormat = FileFormat {
    id: 28_049_467,
    source_type: SourceType::Wikidata,
    name: "DEGAS Elite Compressed, high resolution",
    extensions: &["PC3"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x80, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
