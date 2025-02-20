use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17989653: FileType = FileType {
    file_format: &FileFormat {
        id: 17_989_653,
        source_type: SourceType::Wikidata,
        name: "BOM",
        extensions: &["bom"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x4F, 0x4D, 0x53, 0x74, 0x6F, 0x72, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
