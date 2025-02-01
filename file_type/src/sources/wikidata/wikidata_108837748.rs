use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108837748: FileFormat = FileFormat {
    id: 108_837_748,
    puid: "wikidata/108837748",
    name: "OmniPage Pro Document 11",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
