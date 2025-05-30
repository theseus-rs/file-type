use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853898: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_898,
        source_type: SourceType::Wikidata,
        name: "Ashampoo Burning Studio project",
        extensions: &["ashprj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x73, 0x68, 0x70, 0x72, 0x6A, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
