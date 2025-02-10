use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858822: FileFormat = FileFormat {
    id: 105_858_822,
    source_type: SourceType::Wikidata,
    name: "Interleaf Image bitmap",
    extensions: &["iimg", "img"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x89, 0x4F, 0x50, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
