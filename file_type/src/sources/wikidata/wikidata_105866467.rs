use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866467: FileFormat = FileFormat {
    id: 105_866_467,
    puid: "wikidata/105866467",
    name: "WinAmp/SHOUTcast PlayList",
    extensions: &["pls"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x70, 0x6C, 0x61, 0x79, 0x6C, 0x69, 0x73, 0x74, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
