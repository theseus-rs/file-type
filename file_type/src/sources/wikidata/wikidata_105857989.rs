use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857989: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_989,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine exported player Character (v1.2)",
        extensions: &["chr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x48, 0x52, 0x20, 0x56, 0x31, 0x2E, 0x32,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
