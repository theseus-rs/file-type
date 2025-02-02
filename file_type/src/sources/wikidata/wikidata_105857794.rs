use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857794: FileFormat = FileFormat {
    id: 105_857_794,
    source_type: SourceType::Wikidata,
    name: "Exatron String Floppy virtual wafer image",
    extensions: &["esf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x53, 0x46, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
