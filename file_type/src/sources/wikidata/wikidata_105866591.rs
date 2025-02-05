use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866591: FileFormat = FileFormat {
    id: 105_866_591,
    source_type: SourceType::Wikidata,
    name: "PIMPLE compressed data (v2)",
    extensions: &["pim"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x02, 0x50, 0x49, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
