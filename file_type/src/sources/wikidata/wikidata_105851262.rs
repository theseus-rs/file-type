use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851262: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_262,
        source_type: SourceType::Wikidata,
        name: "Texinfo source",
        extensions: &["texi", "texinfo"],
        media_types: &["application/x-texinfo"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5C, 0x69, 0x6E, 0x70, 0x75, 0x74, 0x20, 0x74, 0x65, 0x78, 0x69, 0x6E,
                        0x66, 0x6F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
