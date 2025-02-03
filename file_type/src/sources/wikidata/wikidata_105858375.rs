use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858375: FileFormat = FileFormat {
    id: 105_858_375,
    source_type: SourceType::Wikidata,
    name: "PonyProg device file",
    extensions: &["e2p"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x32, 0x50, 0x21, 0x4C, 0x61, 0x6E, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
