use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72177065: FileType = FileType {
    file_format: &FileFormat {
        id: 72_177_065,
        source_type: SourceType::Wikidata,
        name: "KiCad Module",
        extensions: &["KICAD_MOD"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x6D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
