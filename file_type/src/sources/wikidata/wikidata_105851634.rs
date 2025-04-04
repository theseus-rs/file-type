use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851634: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_634,
        source_type: SourceType::Wikidata,
        name: "Track Row Column markers data format",
        extensions: &["trc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x61, 0x74, 0x68, 0x46, 0x69, 0x6C, 0x65, 0x54, 0x79, 0x70, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
