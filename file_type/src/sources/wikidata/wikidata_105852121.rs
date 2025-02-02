use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852121: FileFormat = FileFormat {
    id: 105_852_121,
    source_type: SourceType::Wikidata,
    name: "3D Draw SVG image",
    extensions: &["svg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x52, 0x41, 0x57, 0x20, 0x33, 0x2D, 0x44, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
