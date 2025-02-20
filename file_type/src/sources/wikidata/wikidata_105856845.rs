use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856845: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_845,
        source_type: SourceType::Wikidata,
        name: "GenePattern GCT format",
        extensions: &["gct"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x31, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
