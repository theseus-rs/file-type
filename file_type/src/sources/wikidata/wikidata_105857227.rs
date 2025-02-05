use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857227: FileFormat = FileFormat {
    id: 105_857_227,
    source_type: SourceType::Wikidata,
    name: "LigPlot Hydrogen-Bonds data",
    extensions: &["hhb"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6C, 0x69, 0x67, 0x70, 0x6C, 0x6F, 0x74, 0x2E, 0x68, 0x68, 0x62,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
