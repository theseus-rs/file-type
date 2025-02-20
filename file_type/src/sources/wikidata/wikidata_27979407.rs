use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979407: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_407,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Anispec",
        extensions: &["sps"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
