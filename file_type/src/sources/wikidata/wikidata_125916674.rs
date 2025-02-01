use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125916674: FileFormat = FileFormat {
    id: 125_916_674,
    puid: "wikidata/125916674",
    name: "NEC Thermo Tracer Image File TH5100",
    extensions: &["tmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
