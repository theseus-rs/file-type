use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6717445: FileFormat = FileFormat {
    id: 6_717_445,
    puid: "wikidata/6717445",
    name: "MRC",
    extensions: &["mrc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
