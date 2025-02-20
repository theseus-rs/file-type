use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867400: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_400,
        source_type: SourceType::Wikidata,
        name: "Navitel 2.0 Map",
        extensions: &["ntm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x4E, 0x54, 0x4D, 0x07, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
