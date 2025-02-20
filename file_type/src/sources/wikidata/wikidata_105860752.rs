use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860752: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_752,
        source_type: SourceType::Wikidata,
        name: "RenderWare 3d model",
        extensions: &["rwx"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x6F, 0x64, 0x65, 0x6C, 0x42, 0x65, 0x67, 0x69, 0x6E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
