use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1144005: FileFormat = FileFormat {
    id: 1_144_005,
    source_type: SourceType::Wikidata,
    name: "WinHelp",
    extensions: &["hlp"],
    media_types: &["application/winhlp"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
