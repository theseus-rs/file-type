use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858186: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_186,
        source_type: SourceType::Wikidata,
        name: "ICE ECC data (v2.x)",
        extensions: &["ecc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x43, 0x45, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
