use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_355,
        source_type: SourceType::Wikidata,
        name: "EXT1 Extended Amiga Disk image File",
        extensions: &["adf"],
        media_types: &["application/x-amiga-disk-format"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x41, 0x45, 0x2D, 0x2D, 0x41, 0x44, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
