use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837153: FileFormat = FileFormat {
    id: 108_837_153,
    puid: "wikidata/108837153",
    name: "Quicken v4 Data File",
    extensions: &["qdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
