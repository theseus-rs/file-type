use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967199: FileFormat = FileFormat {
    id: 27_967_199,
    puid: "wikidata/27967199",
    name: "Liquid Tracker module",
    extensions: &["liq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
