use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1144005: FileFormat = FileFormat {
    id: 1_144_005,
    source_type: SourceType::Wikidata,
    name: "WinHelp",
    extensions: &["hlp"],
    media_types: &["application/winhlp"],
    signatures: &[Signature {
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
