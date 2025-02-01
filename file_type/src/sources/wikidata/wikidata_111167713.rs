use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111167713: FileFormat = FileFormat {
    id: 111_167_713,
    puid: "wikidata/111167713",
    name: "ACD/CNMR Calculated Spectrum file",
    extensions: &["csp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
