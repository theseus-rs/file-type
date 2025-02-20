use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866122: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_122,
        source_type: SourceType::Wikidata,
        name: "Partially Signed Bitcoin Transaction",
        extensions: &["psbt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x70, 0x73, 0x62, 0x74, 0xFF])],
                },
            }],
        }],
        related_formats: &[],
    },
};
