use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71276559: FileFormat = FileFormat {
    id: 71_276_559,
    puid: "wikidata/71276559",
    name: "PESIM file format",
    extensions: &["pes"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
