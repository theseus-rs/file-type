use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867170: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_170,
        source_type: SourceType::Wikidata,
        name: "PlayStation 3 NPDRM encrypted EDATA",
        extensions: &["pak"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x50, 0x44, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
