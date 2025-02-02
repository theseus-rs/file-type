use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865287: FileFormat = FileFormat {
    id: 105_865_287,
    source_type: SourceType::Wikidata,
    name: "Program Information File (Windows)",
    extensions: &["pif"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x43, 0x52, 0x4F, 0x53, 0x4F, 0x46, 0x54, 0x20, 0x50, 0x49, 0x46,
                    0x45, 0x58,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
