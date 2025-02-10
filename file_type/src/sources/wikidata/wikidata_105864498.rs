use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864498: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_498,
        source_type: SourceType::Wikidata,
        name: "Protein Structure File",
        extensions: &["psf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
