use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_921122: FileFormat = FileFormat {
    id: 921_122,
    puid: "wikidata/921122",
    name: "Java archive",
    extensions: &["jar", "jar"],
    media_types: &["application/java-archive", "application/java-archive"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5F, 0x27, 0xA8, 0x89])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        },
    ],
    related_formats: &[],
};
