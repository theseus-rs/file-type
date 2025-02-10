use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855812: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_812,
        source_type: SourceType::Wikidata,
        name: "Necromancer's Dos Navigator Dialogs",
        extensions: &["dlg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x65, 0x63, 0x72, 0x6F, 0x6D, 0x61, 0x6E, 0x63, 0x65, 0x72, 0x27,
                        0x73, 0x20, 0x44, 0x6F, 0x73, 0x20, 0x4E, 0x61, 0x76, 0x69, 0x67, 0x61,
                        0x74, 0x6F, 0x72, 0x20, 0x44, 0x69, 0x61, 0x6C, 0x6F, 0x67, 0x73, 0x20,
                        0x66, 0x69, 0x6C, 0x65, 0x2E, 0x2E, 0x2E, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
