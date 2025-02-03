use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851072: FileFormat = FileFormat {
    id: 105_851_072,
    source_type: SourceType::Wikidata,
    name: "MultiFax Template",
    extensions: &["tem"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x46, 0x5F, 0x54, 0x45, 0x4D, 0x50, 0x4C, 0x41, 0x54, 0x45, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
