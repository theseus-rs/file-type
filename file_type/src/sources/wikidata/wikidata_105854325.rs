use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854325: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_325,
        source_type: SourceType::Wikidata,
        name: "Aegis Draw plotter definition",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x32, 0x35, 0x38, 0x36, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
