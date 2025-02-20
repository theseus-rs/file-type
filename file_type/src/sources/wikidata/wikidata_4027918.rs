use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4027918: FileType = FileType {
    file_format: &FileFormat {
        id: 4_027_918,
        source_type: SourceType::Wikidata,
        name: "Java Application Descriptor",
        extensions: &["jad"],
        media_types: &["text/vnd.sun.j2me.app-descriptor"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x49, 0x44, 0x6C, 0x65, 0x74, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
