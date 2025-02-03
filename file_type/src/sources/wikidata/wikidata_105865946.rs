use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865946: FileFormat = FileFormat {
    id: 105_865_946,
    source_type: SourceType::Wikidata,
    name: "ProCite data (v5+",
    extensions: &["pdt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xB4, 0xF4, 0x5C, 0x00, 0x0C, 0xC1, 0x9A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
