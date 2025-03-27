use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600458: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_458,
        source_type: SourceType::Wikidata,
        name: "DDI",
        extensions: &["ddi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x44, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x56, 0x65,
                        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
