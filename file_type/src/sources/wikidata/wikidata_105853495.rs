use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853495: FileFormat = FileFormat {
    id: 105_853_495,
    source_type: SourceType::Wikidata,
    name: "Zenographics ZjStream Page Description Language (big-endian)",
    extensions: &["zjs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x5A, 0x4A, 0x5A])],
            },
        }],
    }],
    related_formats: &[],
};
