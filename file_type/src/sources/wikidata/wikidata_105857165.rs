use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857165: FileFormat = FileFormat {
    id: 105_857_165,
    puid: "wikidata/105857165",
    name: "HarmonyCSV format",
    extensions: &["csv"],
    media_types: &["text/csv"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x61, 0x72, 0x6D, 0x6F, 0x6E, 0x79, 0x43, 0x53, 0x56, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
