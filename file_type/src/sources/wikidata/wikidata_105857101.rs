use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857101: FileFormat = FileFormat {
    id: 105_857_101,
    source_type: SourceType::Wikidata,
    name: "STK database update information",
    extensions: &["gd"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x45, 0x47, 0x49, 0x4E, 0x20, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73,
                    0x65, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
