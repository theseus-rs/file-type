use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473282: FileFormat = FileFormat {
    id: 27_473_282,
    puid: "wikidata/27473282",
    name: "CADRG Legend File",
    extensions: &["lgd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
