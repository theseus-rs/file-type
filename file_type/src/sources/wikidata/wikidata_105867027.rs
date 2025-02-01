use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867027: FileFormat = FileFormat {
    id: 105_867_027,
    puid: "wikidata/105867027",
    name: "LigPlot Non-Bonded contacts data",
    extensions: &["nnb"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6C, 0x69, 0x67, 0x70, 0x6C, 0x6F, 0x74, 0x2E, 0x6E, 0x6E, 0x62,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
