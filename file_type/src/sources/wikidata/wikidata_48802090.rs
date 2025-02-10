use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_48802090: FileType = FileType {
    file_format: &FileFormat {
        id: 48_802_090,
        source_type: SourceType::Wikidata,
        name: "Aldus Freehand Drawing, version 3",
        extensions: &["fh3"],
        media_types: &["application/x-freehand"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x48, 0x33, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
