use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27355769: FileFormat = FileFormat {
    id: 27_355_769,
    puid: "wikidata/27355769",
    name: "ADRG Legend Image File",
    extensions: &["lgg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
