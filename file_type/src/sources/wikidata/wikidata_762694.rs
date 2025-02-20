use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_762694: FileType = FileType {
    file_format: &FileFormat {
        id: 762_694,
        source_type: SourceType::Wikidata,
        name: "PostScript Printer Description",
        extensions: &["ppd"],
        media_types: &["application/vnd.cups-ppd"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2A, 0x50, 0x50, 0x44, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
