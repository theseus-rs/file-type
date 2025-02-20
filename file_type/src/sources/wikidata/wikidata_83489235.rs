use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83489235: FileType = FileType {
    file_format: &FileFormat {
        id: 83_489_235,
        source_type: SourceType::Wikidata,
        name: "VisiCalc file format",
        extensions: &["vc", "vcs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
