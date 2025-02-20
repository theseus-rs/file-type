use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860387: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_387,
        source_type: SourceType::Wikidata,
        name: "GROMACS Residue Topology",
        extensions: &["rtp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x20, 0x62, 0x6F, 0x6E, 0x64, 0x65, 0x64, 0x74, 0x79, 0x70, 0x65,
                        0x73, 0x20, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
