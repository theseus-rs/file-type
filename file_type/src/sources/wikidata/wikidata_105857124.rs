use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857124: FileFormat = FileFormat {
    id: 105_857_124,
    source_type: SourceType::Wikidata,
    name: "Generic Feature Format Version 3",
    extensions: &["gff3"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x67, 0x66, 0x66, 0x2D, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x20, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
