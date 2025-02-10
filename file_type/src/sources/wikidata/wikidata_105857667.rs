use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857667: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_667,
        source_type: SourceType::Wikidata,
        name: "EP128Emu tape image",
        extensions: &["tap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x02, 0x75, 0xCD, 0x72, 0x1C, 0x44, 0x51, 0x26, 0x00, 0x00, 0x00, 0x01,
                        0x00, 0x00, 0x5D, 0xC0,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
