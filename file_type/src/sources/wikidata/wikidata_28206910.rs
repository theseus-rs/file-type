use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206910: FileFormat = FileFormat {
    id: 28_206_910,
    source_type: SourceType::Wikidata,
    name: "Portfolio Graphics Compressed",
    extensions: &["pgc", "pgf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x47, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
