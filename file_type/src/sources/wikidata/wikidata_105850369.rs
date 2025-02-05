use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850369: FileFormat = FileFormat {
    id: 105_850_369,
    source_type: SourceType::Wikidata,
    name: "PlayStation 2 system configuration",
    extensions: &["cnf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x4F, 0x4F, 0x54, 0x32, 0x20, 0x3D, 0x20, 0x63, 0x64, 0x72, 0x6F, 0x6D,
                    0x30, 0x3A, 0x5C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
