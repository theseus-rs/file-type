use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851360: FileFormat = FileFormat {
    id: 105_851_360,
    source_type: SourceType::Wikidata,
    name: "Godot Engine Text Resource",
    extensions: &["tres"],
    media_types: &["text/ini"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x67, 0x64, 0x5F, 0x72, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20,
                    0x74, 0x79, 0x70, 0x65, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
