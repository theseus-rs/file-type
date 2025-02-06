use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125916674: FileFormat = FileFormat {
    id: 125_916_674,
    source_type: SourceType::Wikidata,
    name: "NEC Thermo Tracer Image File TH5100",
    extensions: &["tmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
