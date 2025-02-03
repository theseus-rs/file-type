use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855507: FileFormat = FileFormat {
    id: 105_855_507,
    source_type: SourceType::Wikidata,
    name: "Fujitsu composite Font",
    extensions: &["fon"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x55, 0x4A, 0x49, 0x54, 0x53, 0x55, 0x20, 0x43, 0x4F, 0x4D, 0x50, 0x4F,
                    0x53, 0x49, 0x54, 0x45, 0x20, 0x46, 0x4F, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
