use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859883: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_883,
        source_type: SourceType::Wikidata,
        name: "Leitch Native Stream Format video",
        extensions: &["lxf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4C, 0x45, 0x49, 0x54, 0x43, 0x48, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
