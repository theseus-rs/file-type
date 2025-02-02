use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853091: FileFormat = FileFormat {
    id: 105_853_091,
    source_type: SourceType::Wikidata,
    name: "Fetch Script",
    extensions: &["scpt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x61, 0x73, 0x64, 0x55, 0x41, 0x53, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
