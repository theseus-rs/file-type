use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130736862: FileFormat = FileFormat {
    id: 130_736_862,
    puid: "wikidata/130736862",
    name: "Scalate Server Page file",
    extensions: &["ssp"],
    media_types: &["application/x-ssp"],
    internal_signatures: &[],
    related_formats: &[],
};
