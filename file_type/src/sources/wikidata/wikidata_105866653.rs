use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_653,
        source_type: SourceType::Wikidata,
        name: "Psion Serie 5/EPOC Word document",
        extensions: &["psi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10, 0x7F, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
