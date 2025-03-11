use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858742: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_742,
        source_type: SourceType::Wikidata,
        name: "EPOC/Psion Sketch bitmap",
        extensions: &[],
        media_types: &["image/x-epoc-sketch"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x7D, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
