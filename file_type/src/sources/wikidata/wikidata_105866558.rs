use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866558: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_558,
        source_type: SourceType::Wikidata,
        name: "Papyrus Printer Information",
        extensions: &["pri"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x52, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
