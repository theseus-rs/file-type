use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_7839811: FileType = FileType {
    file_format: &FileFormat {
        id: 7_839_811,
        source_type: SourceType::Wikidata,
        name: "TriG",
        extensions: &["trig"],
        media_types: &["application/trig"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
