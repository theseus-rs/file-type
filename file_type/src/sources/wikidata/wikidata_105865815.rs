use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865815: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_815,
        source_type: SourceType::Wikidata,
        name: "Commodore 64 BASIC V2 program",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x08])],
                },
            }],
        }],
        related_formats: &[],
    },
};
