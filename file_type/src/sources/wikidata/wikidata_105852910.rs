use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852910: FileFormat = FileFormat {
    id: 105_852_910,
    source_type: SourceType::Wikidata,
    name: "ScreenSwift Screen Saver Project",
    extensions: &["ssp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x53, 0x77, 0x69, 0x66, 0x74, 0x5D,
                    0x0D, 0x0A, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
