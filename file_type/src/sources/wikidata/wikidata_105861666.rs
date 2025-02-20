use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861666: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_666,
        source_type: SourceType::Wikidata,
        name: "LisaEm configuration",
        extensions: &["lisaem"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6B, 0x65, 0x79, 0x62, 0x6F, 0x61, 0x72, 0x64, 0x69, 0x64, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
