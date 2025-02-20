use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864823: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_823,
        source_type: SourceType::Wikidata,
        name: "PC-File data (gen)",
        extensions: &["hdb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x43, 0x46, 0x3A, 0x64, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
