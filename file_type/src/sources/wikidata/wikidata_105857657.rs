use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857657: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_657,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Markup Language",
        extensions: &["idml"],
        media_types: &["application/vnd.adobe.indesign-idml-package"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
