use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1326659: FileType = FileType {
    file_format: &FileFormat {
        id: 1_326_659,
        source_type: SourceType::Wikidata,
        name: "Virtual Hard Disk",
        extensions: &["vhd"],
        media_types: &[
            "application/x-vhd",
            "application/x-vhd-disk",
            "application/x-virtualbox-vhd",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6E, 0x65, 0x63, 0x74, 0x69, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
