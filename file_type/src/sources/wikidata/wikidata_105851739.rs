use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851739: FileFormat = FileFormat {
    id: 105_851_739,
    source_type: SourceType::Wikidata,
    name: "Phoenix save state (generic)",
    extensions: &["states"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x74, 0x61, 0x74, 0x65, 0x2D, 0x78])],
            },
        }],
    }],
    related_formats: &[],
};
