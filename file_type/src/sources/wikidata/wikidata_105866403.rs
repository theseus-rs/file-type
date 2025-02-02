use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866403: FileFormat = FileFormat {
    id: 105_866_403,
    source_type: SourceType::Wikidata,
    name: "Pro/ENGINEER parts file",
    extensions: &["prt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x55, 0x47, 0x43, 0x3A, 0x32, 0x20, 0x50, 0x41, 0x52, 0x54, 0x20, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
