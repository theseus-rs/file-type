use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860624: FileFormat = FileFormat {
    id: 105_860_624,
    source_type: SourceType::Wikidata,
    name: "Sony PlayStation Resource Container (generic)",
    extensions: &["rco"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x52, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
