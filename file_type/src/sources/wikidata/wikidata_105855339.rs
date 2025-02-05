use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855339: FileFormat = FileFormat {
    id: 105_855_339,
    source_type: SourceType::Wikidata,
    name: "Foto-Mosaic-Edda Data Base Index",
    extensions: &["idx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x4D, 0x49, 0x44, 0x58, 0x44, 0x41, 0x54, 0x41, 0x48, 0x45, 0x41, 0x44,
                    0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
