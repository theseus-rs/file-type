use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55832374: FileFormat = FileFormat {
    id: 55_832_374,
    puid: "wikidata/55832374",
    name: "Event Trace Log file format",
    extensions: &["etl"],
    media_types: &["application/x-ms-etl"],
    internal_signatures: &[],
    related_formats: &[],
};
