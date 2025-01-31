use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850944: FileFormat = FileFormat {
    id: 105_850_944,
    puid: "wikidata/105850944",
    name: "FL Studio Touch Keyboard Form",
    extensions: &["tkp"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x54, 0x6F, 0x75, 0x63, 0x68, 0x4B, 0x65, 0x79, 0x62, 0x46, 0x6F, 0x72,
                    0x6D, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
