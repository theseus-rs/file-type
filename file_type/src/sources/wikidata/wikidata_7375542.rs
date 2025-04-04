use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7375542: FileType = FileType {
    file_format: &FileFormat {
        id: 7_375_542,
        source_type: SourceType::Wikidata,
        name: "Restricted Permission Message",
        extensions: &["rpmsg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0xE8, 0x04, 0x60, 0xC4, 0x11, 0xE3, 0x86,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
