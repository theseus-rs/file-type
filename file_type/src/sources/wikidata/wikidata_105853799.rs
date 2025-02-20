use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853799: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_799,
        source_type: SourceType::Wikidata,
        name: "EPOC/Symbian OPL Application",
        extensions: &["app"],
        media_types: &["application/x-epoc-app"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x37, 0x00, 0x00, 0x10, 0x74, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
