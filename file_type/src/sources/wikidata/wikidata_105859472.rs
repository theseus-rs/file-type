use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859472: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_472,
        source_type: SourceType::Wikidata,
        name: "Quartus Workspace",
        extensions: &["qws"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x57, 0x6F, 0x72, 0x6B,
                        0x73, 0x70, 0x61, 0x63, 0x65, 0x5D, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
