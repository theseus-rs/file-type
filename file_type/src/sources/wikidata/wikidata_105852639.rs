use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852639: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_639,
        source_type: SourceType::Wikidata,
        name: "Spectrum Prog snapshot",
        extensions: &["spg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x70, 0x65, 0x63, 0x74, 0x72, 0x75, 0x6D, 0x50, 0x72, 0x6F, 0x67,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
