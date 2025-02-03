use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857357: FileFormat = FileFormat {
    id: 105_857_357,
    source_type: SourceType::Wikidata,
    name: "Easy CD Creator's label (v5)",
    extensions: &["jwl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x00, 0x4F, 0x00, 0x58, 0x00, 0x49, 0x00, 0x20, 0x00, 0x35, 0x00, 0x2E,
                    0x00, 0x30, 0x00, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
