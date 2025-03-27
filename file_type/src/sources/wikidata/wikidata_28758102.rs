use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28758102: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_102,
        source_type: SourceType::Wikidata,
        name: "InstallShield CAB",
        extensions: &["cab", "hdr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x53, 0x63, 0x28])],
                },
            }],
        }],
        related_formats: &[],
    },
};
