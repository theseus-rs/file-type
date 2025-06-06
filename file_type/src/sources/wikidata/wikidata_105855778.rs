use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855778: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_778,
        source_type: SourceType::Wikidata,
        name: "Telepaint printer Driver",
        extensions: &["drv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x32, 0x01, 0x00, 0x4F, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
