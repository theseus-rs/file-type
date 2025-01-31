use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849834: FileFormat = FileFormat {
    id: 105_849_834,
    puid: "wikidata/105849834",
    name: "CFast Animation (secured)",
    extensions: &["cft"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x44, 0x59, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
