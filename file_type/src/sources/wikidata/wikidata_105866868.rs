use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866868: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_868,
        source_type: SourceType::Wikidata,
        name: "Cisco VPN Profile Configuration File",
        extensions: &["pcf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5B, 0x6D, 0x61, 0x69, 0x6E, 0x5D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
