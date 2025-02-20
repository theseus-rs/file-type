use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205699: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_699,
        source_type: SourceType::Wikidata,
        name: "Aperio SVS",
        extensions: &["svs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
