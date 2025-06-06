use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858944: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_944,
        source_type: SourceType::Wikidata,
        name: "MultiArtist bitmap",
        extensions: &["mg1", "mg2", "mg4", "mg8"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x47, 0x48, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
