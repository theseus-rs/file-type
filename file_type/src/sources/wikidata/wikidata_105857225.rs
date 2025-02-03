use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857225: FileFormat = FileFormat {
    id: 105_857_225,
    source_type: SourceType::Wikidata,
    name: "Peter Norton Computing Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4E, 0x43, 0x49, 0x42, 0x48, 0x44, 0x4D, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
