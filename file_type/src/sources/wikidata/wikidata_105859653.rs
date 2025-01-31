use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859653: FileFormat = FileFormat {
    id: 105_859_653,
    puid: "wikidata/105859653",
    name: "Reaper video",
    extensions: &["fmv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x52, 0x65, 0x61, 0x70, 0x65, 0x72, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
