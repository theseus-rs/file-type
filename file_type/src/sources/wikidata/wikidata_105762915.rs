use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762915: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_915,
        source_type: SourceType::Wikidata,
        name: "Lotus Translation Table",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD2, 0x04, 0x94, 0x26])],
                },
            }],
        }],
        related_formats: &[],
    },
};
