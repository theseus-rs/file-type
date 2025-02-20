use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865055: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_055,
        source_type: SourceType::Wikidata,
        name: "TM-MARS compiled Redcode program",
        extensions: &["prg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x45, 0x44, 0x43, 0x4F, 0x44, 0x45, 0x20, 0x50, 0x52, 0x4F, 0x47,
                        0x52, 0x41, 0x4D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
