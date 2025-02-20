use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855246: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_246,
        source_type: SourceType::Wikidata,
        name: "Floppy Disk Manager disk image",
        extensions: &["fdm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x46, 0x44, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
