use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855042: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_042,
        source_type: SourceType::Wikidata,
        name: "Kexis lossless compressed audio",
        extensions: &["kxs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x45, 0x58, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
