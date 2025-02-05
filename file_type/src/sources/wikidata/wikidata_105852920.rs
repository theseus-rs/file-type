use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852920: FileFormat = FileFormat {
    id: 105_852_920,
    source_type: SourceType::Wikidata,
    name: "ArtCAM STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["model/x.stl-binary"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x72, 0x74, 0x43, 0x41, 0x4D, 0x20, 0x53, 0x54, 0x4C, 0x20, 0x46, 0x69,
                    0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
