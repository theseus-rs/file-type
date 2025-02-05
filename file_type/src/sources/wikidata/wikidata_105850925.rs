use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850925: FileFormat = FileFormat {
    id: 105_850_925,
    source_type: SourceType::Wikidata,
    name: "LTP Nuclear ZX tape image",
    extensions: &["ltp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x11, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
