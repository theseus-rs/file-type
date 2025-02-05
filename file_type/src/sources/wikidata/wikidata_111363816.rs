use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363816: FileFormat = FileFormat {
    id: 111_363_816,
    source_type: SourceType::Wikidata,
    name: "Raw Yamaha 4-bit ADPCM format data",
    extensions: &["yadpcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
