use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856886: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_886,
        source_type: SourceType::Wikidata,
        name: "Sensible Golf game save (Amiga)",
        extensions: &["glf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x4A, 0x31, 0x38, 0x30, 0x37, 0x31, 0x33, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
