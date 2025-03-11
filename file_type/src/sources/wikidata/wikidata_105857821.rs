use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857821: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_821,
        source_type: SourceType::Wikidata,
        name: "Interword printer definition",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x54, 0x4B, 0x50, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
