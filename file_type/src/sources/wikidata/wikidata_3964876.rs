use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3964876: FileType = FileType {
    file_format: &FileFormat {
        id: 3_964_876,
        source_type: SourceType::Wikidata,
        name: "Pentax Electronic File",
        extensions: &["pef", "ptx"],
        media_types: &["image/x-pentax-pef", "image/x-raw-pentax"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x00, 0x2A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
