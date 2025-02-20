use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59616412: FileType = FileType {
    file_format: &FileFormat {
        id: 59_616_412,
        source_type: SourceType::Wikidata,
        name: "CRT C64 Cartridge Image Format",
        extensions: &["crt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x36, 0x34, 0x20, 0x43, 0x41, 0x52, 0x54, 0x52, 0x49, 0x44, 0x47,
                        0x45, 0x20, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
