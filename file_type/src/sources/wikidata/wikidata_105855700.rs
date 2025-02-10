use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855700: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_700,
        source_type: SourceType::Wikidata,
        name: "OpenVPN profile (with rem)",
        extensions: &["ovpn"],
        media_types: &["application/x-openvpn-profile"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
