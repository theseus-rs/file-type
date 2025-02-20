use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_11241282: FileType = FileType {
    file_format: &FileFormat {
        id: 11_241_282,
        source_type: SourceType::Wikidata,
        name: "RPM",
        extensions: &["rpm"],
        media_types: &["application/x-redhat-package-manager", "application/x-rpm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xED, 0xAB, 0xEE, 0xDB])],
                },
            }],
        }],
        related_formats: &[],
    },
};
