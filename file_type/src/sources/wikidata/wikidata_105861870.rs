use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861870: FileFormat = FileFormat {
    id: 105_861_870,
    source_type: SourceType::Wikidata,
    name: "Mesa 2 spreadsheet",
    extensions: &["m2"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x65, 0x73, 0x61, 0x20, 0x43, 0x4D, 0x50, 0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
