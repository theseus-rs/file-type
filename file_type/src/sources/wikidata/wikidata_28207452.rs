use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207452: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_452,
        source_type: SourceType::Wikidata,
        name: "Vista Data File Format",
        extensions: &["v"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x2D, 0x64, 0x61, 0x74, 0x61, 0x20, 0x32, 0x20, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
