use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859791: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_791,
        source_type: SourceType::Wikidata,
        name: "VistaDB database",
        extensions: &["vdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x04, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
