use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857750: FileFormat = FileFormat {
    id: 105_857_750,
    source_type: SourceType::Wikidata,
    name: "LDBS disk image (-v0.2)",
    extensions: &["ldbs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x42, 0x53, 0x01, 0x44, 0x53, 0x4B, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
