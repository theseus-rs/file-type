use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866436: FileFormat = FileFormat {
    id: 105_866_436,
    puid: "wikidata/105866436",
    name: "Panzerkrieg for Windows Scenario",
    extensions: &["pks"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x46, 0x57, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
