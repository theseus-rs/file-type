use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_653,
        source_type: SourceType::Wikidata,
        name: "Bagpipe notation",
        extensions: &["bww"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x61, 0x67, 0x70, 0x69, 0x70, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
