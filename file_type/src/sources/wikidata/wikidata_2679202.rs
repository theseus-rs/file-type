use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2679202: FileFormat = FileFormat {
    id: 2_679_202,
    puid: "wikidata/2679202",
    name: "nds",
    extensions: &["nds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
