use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600483: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_483,
        source_type: SourceType::Wikidata,
        name: "DSK (CPCEMU)",
        extensions: &["dsk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x56, 0x20, 0x2D, 0x20, 0x43, 0x50, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
