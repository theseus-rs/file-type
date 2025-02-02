use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52834888: FileFormat = FileFormat {
    id: 52_834_888,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel 2.x Worksheet",
    extensions: &["xls"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x09, 0x00, 0x04, 0x00, 0x02, 0x00, 0x10, 0x00, 0x0B, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
