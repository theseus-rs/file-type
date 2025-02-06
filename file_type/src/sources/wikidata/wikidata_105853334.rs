use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853334: FileFormat = FileFormat {
    id: 105_853_334,
    source_type: SourceType::Wikidata,
    name: "sBOX meta-file format",
    extensions: &["sbox"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x62, 0x30, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
