use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856423: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_423,
        source_type: SourceType::Wikidata,
        name: "EPOC Word document",
        extensions: &["wrd"],
        media_types: &["application/x-epoc-word"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x7F, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
