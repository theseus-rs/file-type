use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858688: FileFormat = FileFormat {
    id: 105_858_688,
    source_type: SourceType::Wikidata,
    name: "Boundary Scan Description Language (with rem)",
    extensions: &["bsdl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
