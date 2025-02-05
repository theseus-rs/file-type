use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867013: FileFormat = FileFormat {
    id: 105_867_013,
    source_type: SourceType::Wikidata,
    name: "ZX Spectrum Next binary format (v1.2)",
    extensions: &["nex"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x65, 0x78, 0x74, 0x56, 0x31, 0x2E, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
