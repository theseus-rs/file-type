use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858038: FileFormat = FileFormat {
    id: 105_858_038,
    source_type: SourceType::Wikidata,
    name: "SIDPLAY Info",
    extensions: &["sid"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x49, 0x44, 0x50, 0x4C, 0x41, 0x59, 0x20, 0x49, 0x4E, 0x46, 0x4F, 0x46,
                    0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
