use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855442: FileFormat = FileFormat {
    id: 105_855_442,
    source_type: SourceType::Wikidata,
    name: "Cleanersoft Free Hide Folder data",
    extensions: &["fhf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x72, 0x65, 0x65, 0x20, 0x48, 0x69, 0x64, 0x65, 0x20, 0x46, 0x6F, 0x6C,
                    0x64, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
