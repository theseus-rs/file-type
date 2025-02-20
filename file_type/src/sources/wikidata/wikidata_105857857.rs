use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857857: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_857,
        source_type: SourceType::Wikidata,
        name: "INDENICA Variability Modelling Language",
        extensions: &["ivml"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
