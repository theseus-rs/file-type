use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858217: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_217,
        source_type: SourceType::Wikidata,
        name: "Encapsulated PostScript Interchange",
        extensions: &["epsi"],
        media_types: &["application/postscript"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
