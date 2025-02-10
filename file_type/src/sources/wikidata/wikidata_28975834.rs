use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28975834: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_834,
        source_type: SourceType::Wikidata,
        name: "Tripos MOL2 molecule file",
        extensions: &["mol2"],
        media_types: &["chemical/x-mol2"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23])],
                },
            }],
        }],
        related_formats: &[],
    },
};
