use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859398: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_398,
        source_type: SourceType::Wikidata,
        name: "Micrografx QuickSilver graphic plugin",
        extensions: &["qsf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x45, 0x53, 0x49, 0x47, 0x4E, 0x45, 0x52, 0x44, 0x4F, 0x43, 0x00,
                        0x00, 0x08, 0x10, 0x02, 0x78, 0x9C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
