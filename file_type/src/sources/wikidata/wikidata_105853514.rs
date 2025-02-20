use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853514: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_514,
        source_type: SourceType::Wikidata,
        name: "ZVR eBook",
        extensions: &["zvr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x21, 0x43, 0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64,
                        0x21, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
