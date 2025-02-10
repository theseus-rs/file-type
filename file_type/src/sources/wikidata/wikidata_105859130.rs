use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859130: FileFormat = FileFormat {
    id: 105_859_130,
    source_type: SourceType::Wikidata,
    name: "HSI Raw bitmap",
    extensions: &["hst", "raw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6D, 0x68, 0x77, 0x61, 0x6E, 0x68])],
            },
        }],
    }],
    related_formats: &[],
};
