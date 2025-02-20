use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855795: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_795,
        source_type: SourceType::Wikidata,
        name: "CP Backup Directory (v7.x)",
        extensions: &["dir"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x44, 0x43, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
