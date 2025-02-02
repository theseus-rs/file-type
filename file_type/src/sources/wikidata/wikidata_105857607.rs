use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857607: FileFormat = FileFormat {
    id: 105_857_607,
    source_type: SourceType::Wikidata,
    name: "ISI gMotor MAS type 1 game data archive",
    extensions: &["mas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xC8, 0xCF, 0xD2, 0xD8, 0xCE, 0xD8, 0xE6, 0xC9, 0xCA, 0xDD, 0xD8, 0xBE, 0xBB,
                    0xA6, 0xBF, 0x90,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
