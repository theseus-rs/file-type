use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859444: FileFormat = FileFormat {
    id: 105_859_444,
    source_type: SourceType::Wikidata,
    name: "Qt Message",
    extensions: &["qm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0xB8, 0x64, 0x18, 0xCA, 0xEF, 0x9C, 0x95, 0xCD, 0x21, 0x1C, 0xBF, 0x60,
                    0xA1, 0xBD, 0xDD,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
