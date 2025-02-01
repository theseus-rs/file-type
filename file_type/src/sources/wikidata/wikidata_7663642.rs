use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7663642: FileFormat = FileFormat {
    id: 7_663_642,
    puid: "wikidata/7663642",
    name: "System Deployment Image",
    extensions: &["sdi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x53, 0x44, 0x49, 0x30, 0x30, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
