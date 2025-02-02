use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865836: FileFormat = FileFormat {
    id: 105_865_836,
    source_type: SourceType::Wikidata,
    name: "Artisoft installation Package",
    extensions: &["pak"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x52, 0x54, 0x49, 0x50, 0x41, 0x43, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
