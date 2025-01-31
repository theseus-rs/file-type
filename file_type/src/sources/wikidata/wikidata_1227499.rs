use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1227499: FileFormat = FileFormat {
    id: 1_227_499,
    puid: "wikidata/1227499",
    name: "Direct Stream Digital",
    extensions: &["dsf"],
    media_types: &["audio/x-dsf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x44, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
