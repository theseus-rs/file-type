use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758103: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_103,
        source_type: SourceType::Wikidata,
        name: "InstallShield Z",
        extensions: &["z"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x13, 0x5D, 0x65, 0x8C, 0x3A, 0x01, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
