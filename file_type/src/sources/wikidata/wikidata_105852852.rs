use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852852: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_852,
        source_type: SourceType::Wikidata,
        name: "Microsoft C/C++ project Status info",
        extensions: &["sts"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x2D, 0x5D, 0x0D, 0x0A, 0x09,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
