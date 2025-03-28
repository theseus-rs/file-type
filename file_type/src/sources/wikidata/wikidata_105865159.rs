use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865159: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_159,
        source_type: SourceType::Wikidata,
        name: "Commodore VIC-20 BASIC V2 program (8K RAM expansion)",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x12])],
                },
            }],
        }],
        related_formats: &[],
    },
};
