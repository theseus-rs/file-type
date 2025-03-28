use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857025: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_025,
        source_type: SourceType::Wikidata,
        name: "GML Feature Class List",
        extensions: &["gfs"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x47, 0x4D, 0x4C, 0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x43,
                        0x6C, 0x61, 0x73, 0x73, 0x4C, 0x69, 0x73, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
