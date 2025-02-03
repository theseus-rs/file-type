use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865866: FileFormat = FileFormat {
    id: 105_865_866,
    source_type: SourceType::Wikidata,
    name: "Ambient Design ArtRage project",
    extensions: &["ptg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x6D, 0x62, 0x69, 0x65, 0x6E, 0x74, 0x20, 0x44, 0x65, 0x73, 0x69, 0x67,
                    0x6E, 0x20, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x66, 0x69, 0x6C,
                    0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
