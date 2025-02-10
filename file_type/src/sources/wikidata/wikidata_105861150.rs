use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861150: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_150,
        source_type: SourceType::Wikidata,
        name: "Logisim memory image",
        extensions: &["ram", "rom"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0x32, 0x2E, 0x30, 0x20, 0x72, 0x61, 0x77,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
