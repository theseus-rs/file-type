use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852968: FileFormat = FileFormat {
    id: 105_852_968,
    source_type: SourceType::Wikidata,
    name: "StarLogo TNG Project",
    extensions: &["sltng"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x74, 0x61, 0x72, 0x4C, 0x6F, 0x67, 0x6F, 0x54, 0x4E, 0x47, 0x20, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
