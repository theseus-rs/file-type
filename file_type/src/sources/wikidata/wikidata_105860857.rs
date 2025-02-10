use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860857: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_857,
        source_type: SourceType::Wikidata,
        name: "Samplitude RAM Project (old ver)",
        extensions: &["rap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x4B, 0x44, 0x53, 0x41, 0x4D, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
