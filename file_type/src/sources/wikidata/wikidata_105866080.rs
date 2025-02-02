use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866080: FileFormat = FileFormat {
    id: 105_866_080,
    source_type: SourceType::Wikidata,
    name: "Commodore VIC-20 BASIC V2 program",
    extensions: &["prg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x10])],
            },
        }],
    }],
    related_formats: &[],
};
