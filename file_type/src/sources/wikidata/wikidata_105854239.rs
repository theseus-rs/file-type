use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854239: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_239,
        source_type: SourceType::Wikidata,
        name: "OptionSoft WindowsXCompressor archive",
        extensions: &["gcf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x47, 0x43, 0x46, 0x00, 0x00, 0x00, 0x20, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
