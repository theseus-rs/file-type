use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857591: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_591,
        source_type: SourceType::Wikidata,
        name: "Avidemux video Index",
        extensions: &["idx2"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x53, 0x44, 0x31, 0x0A, 0x5B, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D,
                        0x5D, 0x0A, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
