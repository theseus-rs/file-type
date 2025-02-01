use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113577664: FileFormat = FileFormat {
    id: 113_577_664,
    puid: "wikidata/113577664",
    name: "Philips/OptImage's Master tool",
    extensions: &["cd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
