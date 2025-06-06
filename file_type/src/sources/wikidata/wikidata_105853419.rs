use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853419: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_419,
        source_type: SourceType::Wikidata,
        name: "BorgBackup data segment",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4F, 0x52, 0x47, 0x5F, 0x53, 0x45, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
