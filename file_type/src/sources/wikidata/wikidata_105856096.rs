use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856096: FileFormat = FileFormat {
    id: 105_856_096,
    puid: "wikidata/105856096",
    name: "Datel Action Replay for Windows 95/98 cheat data",
    extensions: &["dc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x64, 0x63, 0x31, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
