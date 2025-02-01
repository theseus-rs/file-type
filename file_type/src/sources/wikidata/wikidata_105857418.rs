use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857418: FileFormat = FileFormat {
    id: 105_857_418,
    puid: "wikidata/105857418",
    name: "Ways Job Control",
    extensions: &["joc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x6F, 0x62, 0x57, 0x69, 0x6E])],
            },
        }],
    }],
    related_formats: &[],
};
