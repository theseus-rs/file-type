use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1047541: FileType = FileType {
    file_format: &FileFormat {
        id: 1_047_541,
        source_type: SourceType::Wikidata,
        name: "Encapsulated PostScript",
        extensions: &["eps"],
        media_types: &["application/postscript", "image/x-eps"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xC5, 0xD0, 0xD3, 0xC6])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x33,
                            0x2E, 0x30, 0x20, 0x45, 0x50, 0x53, 0x46, 0x2D, 0x33, 0x20, 0x30,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
