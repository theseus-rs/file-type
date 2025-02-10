use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207296: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_296,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Extended",
        extensions: &["spx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x50, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
