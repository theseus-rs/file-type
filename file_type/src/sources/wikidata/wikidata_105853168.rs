use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853168: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_168,
        source_type: SourceType::Wikidata,
        name: "Proton language Schema",
        extensions: &["sch"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x54, 0x50, 0x65, 0x72, 0x73,
                        0x48, 0x6F, 0x6C, 0x64, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
