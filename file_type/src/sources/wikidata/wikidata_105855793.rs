use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855793: FileFormat = FileFormat {
    id: 105_855_793,
    puid: "wikidata/105855793",
    name: "dBASE compiled Object program",
    extensions: &["dbo"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x05, 0x44, 0x42, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};
