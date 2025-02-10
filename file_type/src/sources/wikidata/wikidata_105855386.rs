use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855386: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_386,
        source_type: SourceType::Wikidata,
        name: "Freenet node Reference (var.2)",
        extensions: &["fref"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x69, 0x64, 0x65, 0x6E, 0x74, 0x69, 0x74, 0x79, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
