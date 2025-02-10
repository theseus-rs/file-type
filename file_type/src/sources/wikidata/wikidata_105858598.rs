use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858598: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_598,
        source_type: SourceType::Wikidata,
        name: "Breevy text snippet",
        extensions: &["bvy"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x42, 0x72, 0x65, 0x65, 0x76, 0x79, 0x20, 0x6E, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
