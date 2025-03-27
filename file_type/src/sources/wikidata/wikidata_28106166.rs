use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28106166: FileType = FileType {
    file_format: &FileFormat {
        id: 28_106_166,
        source_type: SourceType::Wikidata,
        name: "Psion PIC",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x43, 0xDC])],
                },
            }],
        }],
        related_formats: &[],
    },
};
