use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60371443: FileFormat = FileFormat {
    id: 60_371_443,
    puid: "wikidata/60371443",
    name: "Quark Xpress Report File",
    extensions: &["qxp_report"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
