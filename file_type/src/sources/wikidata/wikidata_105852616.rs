use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852616: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_616,
        source_type: SourceType::Wikidata,
        name: "Genesis - The Third Day Script",
        extensions: &["scrpt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x72, 0x61, 0x77, 0x4D, 0x6F, 0x64, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
