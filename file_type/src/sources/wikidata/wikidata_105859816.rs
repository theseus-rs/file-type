use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859816: FileFormat = FileFormat {
    id: 105_859_816,
    source_type: SourceType::Wikidata,
    name: "Surpass Spreadsheet",
    extensions: &["vts"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x09, 0x04, 0x06, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0xF7, 0x7E, 0x18,
                    0x00, 0x92, 0x00, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
