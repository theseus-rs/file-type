use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858640: FileFormat = FileFormat {
    id: 105_858_640,
    source_type: SourceType::Wikidata,
    name: "Microsoft QuickBASIC 4.5 tokenized source",
    extensions: &["bas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFC, 0x00, 0x01, 0x00, 0x0C, 0x00, 0x81, 0x01, 0x82, 0x01, 0x06, 0x00, 0x01,
                    0x02, 0x03, 0x04, 0x05, 0x08, 0x10,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
