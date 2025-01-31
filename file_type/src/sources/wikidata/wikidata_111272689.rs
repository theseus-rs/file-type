use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111272689: FileFormat = FileFormat {
    id: 111_272_689,
    puid: "wikidata/111272689",
    name: "Farandoyle linear module format",
    extensions: &["f2r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
