use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867407: FileFormat = FileFormat {
    id: 105_867_407,
    source_type: SourceType::Wikidata,
    name: "CharlieCalc spreadsheet",
    extensions: &["nw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x68, 0x61, 0x72, 0x6C, 0x69, 0x65, 0x43, 0x61, 0x6C, 0x63, 0x20, 0x20,
                    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
