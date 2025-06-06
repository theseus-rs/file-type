use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850831: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_831,
        source_type: SourceType::Wikidata,
        name: "KiXtart tokenized script",
        extensions: &["kx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0xAF, 0x06, 0x00, 0x00, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
