use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850934: FileFormat = FileFormat {
    id: 105_850_934,
    puid: "wikidata/105850934",
    name: "TopSolid Project",
    extensions: &["topprj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x08, 0x54, 0x6F, 0x70, 0x53, 0x6F, 0x6C, 0x69, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
