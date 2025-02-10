use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28771321: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_321,
        source_type: SourceType::Wikidata,
        name: "Microsoft Update Standalone Package",
        extensions: &["msu"],
        media_types: &["application/vnd.ms-cab-compressed", "text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
