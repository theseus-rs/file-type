use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127604990: FileFormat = FileFormat {
    id: 127_604_990,
    puid: "wikidata/127604990",
    name: "Awk script",
    extensions: &["awk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
