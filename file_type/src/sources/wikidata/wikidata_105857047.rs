use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857047: FileFormat = FileFormat {
    id: 105_857_047,
    source_type: SourceType::Wikidata,
    name: "GameShark SharkSave for GameCube",
    extensions: &["gcs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x43, 0x53, 0x41, 0x56, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
