use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859103: FileFormat = FileFormat {
    id: 105_859_103,
    source_type: SourceType::Wikidata,
    name: "Graph2Font bitmap (zlib compressed)",
    extensions: &["g2f"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x32, 0x46, 0x5A, 0x4C, 0x49, 0x42, 0x78,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
