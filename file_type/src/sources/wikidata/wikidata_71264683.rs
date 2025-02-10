use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71264683: FileFormat = FileFormat {
    id: 71_264_683,
    source_type: SourceType::Wikidata,
    name: "Hippel module",
    extensions: &["hip"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x60])],
            },
        }],
    }],
    related_formats: &[],
};
