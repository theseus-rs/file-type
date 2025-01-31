use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272306: FileFormat = FileFormat {
    id: 111_272_306,
    puid: "wikidata/111272306",
    name: "Ensoniq EPS instrument file",
    extensions: &["efe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
