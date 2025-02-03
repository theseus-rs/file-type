use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363816: FileFormat = FileFormat {
    id: 111_363_816,
    source_type: SourceType::Wikidata,
    name: "Raw Yamaha 4-bit ADPCM format data",
    extensions: &["yadpcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
