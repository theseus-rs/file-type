use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857671: FileFormat = FileFormat {
    id: 105_857_671,
    source_type: SourceType::Wikidata,
    name: "Mahalito 2D disk image",
    extensions: &["2d"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x32, 0x44, 0x20, 0x20, 0x76, 0x65, 0x72, 0x31, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
