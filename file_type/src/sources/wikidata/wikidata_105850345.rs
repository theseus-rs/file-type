use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850345: FileFormat = FileFormat {
    id: 105_850_345,
    source_type: SourceType::Wikidata,
    name: "PeachCalc spreadsheet",
    extensions: &["cal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x65, 0x61, 0x63, 0x68, 0x43, 0x61, 0x6C, 0x63, 0x20, 0x76, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
