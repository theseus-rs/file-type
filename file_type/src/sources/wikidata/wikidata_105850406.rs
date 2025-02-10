use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850406: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_406,
        source_type: SourceType::Wikidata,
        name: "ep32 Configuration",
        extensions: &["cfg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x50, 0x5F, 0x50, 0x41, 0x47, 0x45, 0x3D, 0x23,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
