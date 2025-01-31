use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850293: FileFormat = FileFormat {
    id: 105_850_293,
    puid: "wikidata/105850293",
    name: "CSI MaRKup drawing",
    extensions: &["mrk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x41, 0x54, 0x48, 0x65, 0x61, 0x64, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
