use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205519: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_519,
        source_type: SourceType::Wikidata,
        name: "ICDRAW Single Icon File",
        extensions: &["ibi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x43, 0x42, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
