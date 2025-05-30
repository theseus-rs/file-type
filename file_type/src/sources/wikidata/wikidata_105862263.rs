use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862263: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_263,
        source_type: SourceType::Wikidata,
        name: "Cabri 3D Macro",
        extensions: &["mac"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x61, 0x63, 0x72, 0x6F, 0x20, 0x43, 0x61, 0x62, 0x72, 0x69, 0x49,
                        0x49, 0x20, 0x76, 0x65, 0x72, 0x73, 0x2E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
