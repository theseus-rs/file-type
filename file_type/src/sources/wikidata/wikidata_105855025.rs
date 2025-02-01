use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855025: FileFormat = FileFormat {
    id: 105_855_025,
    puid: "wikidata/105855025",
    name: "SGA game data archive",
    extensions: &["sga"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x41, 0x52, 0x43, 0x48, 0x49, 0x56, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
