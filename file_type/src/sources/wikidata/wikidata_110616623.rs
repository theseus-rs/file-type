use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110616623: FileType = FileType {
    file_format: &FileFormat {
        id: 110_616_623,
        source_type: SourceType::Wikidata,
        name: "EMSA-MAS Spectral Data",
        extensions: &["msa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x4D, 0x53, 0x41, 0x2F, 0x4D, 0x41, 0x53, 0x20, 0x53, 0x50, 0x45,
                        0x43, 0x54, 0x52, 0x41, 0x4C, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20, 0x46,
                        0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
