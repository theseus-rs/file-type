use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853260: FileFormat = FileFormat {
    id: 105_853_260,
    puid: "wikidata/105853260",
    name: "FileGateway Server configuration",
    extensions: &["prefs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x52, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
