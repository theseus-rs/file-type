use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862062: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_062,
        source_type: SourceType::Wikidata,
        name: "PSGMOD module",
        extensions: &["psgmod"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x47, 0x4D, 0x4F, 0x44, 0x2D, 0x32, 0x48, 0x45, 0x41, 0x44,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
