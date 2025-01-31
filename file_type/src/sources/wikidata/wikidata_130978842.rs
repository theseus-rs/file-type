use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130978842: FileFormat = FileFormat {
    id: 130_978_842,
    puid: "wikidata/130978842",
    name: "Slash file format",
    extensions: &["sla"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
