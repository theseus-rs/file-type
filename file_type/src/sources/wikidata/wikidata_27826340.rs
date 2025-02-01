use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826340: FileFormat = FileFormat {
    id: 27_826_340,
    puid: "wikidata/27826340",
    name: "Auxiliary file, AUX variant",
    extensions: &["aux"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
