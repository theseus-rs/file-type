use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864652: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_652,
        source_type: SourceType::Wikidata,
        name: "Casio Prizm data (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xAA, 0xAC, 0xBD, 0xAF, 0x90, 0x88, 0x9A, 0x8D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
