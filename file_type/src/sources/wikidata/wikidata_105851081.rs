use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851081: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_081,
        source_type: SourceType::Wikidata,
        name: "Adobe PageMaker Template (v6.5)",
        extensions: &["t65"],
        media_types: &["application/x-pagemaker"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
