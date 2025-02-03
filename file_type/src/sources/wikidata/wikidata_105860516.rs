use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860516: FileFormat = FileFormat {
    id: 105_860_516,
    source_type: SourceType::Wikidata,
    name: "Rigaku XRD RAS format",
    extensions: &["ras"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x52, 0x41, 0x53, 0x5F, 0x44, 0x41, 0x54, 0x41, 0x5F, 0x53, 0x54, 0x41,
                    0x52, 0x54, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
