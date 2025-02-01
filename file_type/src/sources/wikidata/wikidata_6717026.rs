use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6717026: FileFormat = FileFormat {
    id: 6_717_026,
    puid: "wikidata/6717026",
    name: "MOI",
    extensions: &["moi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
