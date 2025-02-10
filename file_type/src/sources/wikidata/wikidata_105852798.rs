use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852798: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_798,
        source_type: SourceType::Wikidata,
        name: "Microsoft Solitaire Saved game",
        extensions: &["solitairesave-ms"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x47, 0x4D, 0x48, 0x01, 0x00, 0x00, 0x00, 0x28, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
