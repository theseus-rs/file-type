use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861869: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_869,
        source_type: SourceType::Wikidata,
        name: "Microsoft Delta Compression format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x43, 0x4D, 0x01, 0x50, 0x41, 0x33, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
