use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000588: FileFormat = FileFormat {
    id: 29_000_588,
    source_type: SourceType::Wikidata,
    name: "Microsoft memory dump",
    extensions: &["dmp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x47, 0x45, 0x44, 0x55, 0x4D, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
