use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109616958: FileFormat = FileFormat {
    id: 109_616_958,
    source_type: SourceType::Wikidata,
    name: "HydroCAD Rainfall table",
    extensions: &["hcr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2F, 0x2F, 0x20, 0x48, 0x79, 0x64, 0x72, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x52,
                    0x61, 0x69, 0x6E, 0x66, 0x61, 0x6C, 0x6C, 0x20, 0x74, 0x61, 0x62, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
