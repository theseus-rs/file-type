use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852069: FileFormat = FileFormat {
    id: 105_852_069,
    source_type: SourceType::Wikidata,
    name: "Excelsior Phase Two saved game",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x78, 0x32, 0x20, 0x73, 0x61, 0x76, 0x65, 0x20, 0x76, 0x65, 0x72, 0x20,
                    0x32, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
