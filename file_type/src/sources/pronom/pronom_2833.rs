use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2833: FileFormat = FileFormat {
    id: 2_833,
    source_type: SourceType::Pronom,
    name: "Solidworks Design Document Files",
    extensions: &["sldprt", "slddrw", "sldasm", "sld", "sldlfp", "slddrt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x00, 0x04]),
                    Token::WildcardCountRange(0, 2_048),
                    Token::Literal(&[0x34, 0xF6, 0xE6, 0x47, 0x56, 0xE6, 0x47, 0x37, 0xF2]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
