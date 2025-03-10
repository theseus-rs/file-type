use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864669: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_669,
        source_type: SourceType::Wikidata,
        name: "PA-RISC 2.0 object code (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x14, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
