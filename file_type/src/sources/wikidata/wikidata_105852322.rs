use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852322: FileFormat = FileFormat {
    id: 105_852_322,
    puid: "wikidata/105852322",
    name: "Slim Show project",
    extensions: &["ss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x73, 0x70, 0x66])],
            },
        }],
    }],
    related_formats: &[],
};
