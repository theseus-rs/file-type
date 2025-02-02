use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105849972: FileFormat = FileFormat {
    id: 105_849_972,
    source_type: SourceType::Wikidata,
    name: "COMX-35 program",
    extensions: &["comx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x4D, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
