use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75717634: FileFormat = FileFormat {
    id: 75_717_634,
    puid: "wikidata/75717634",
    name: "Corel Photo Paint User Defined Filter",
    extensions: &["usr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
