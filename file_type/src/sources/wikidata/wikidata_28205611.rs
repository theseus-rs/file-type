use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205611: FileFormat = FileFormat {
    id: 28_205_611,
    puid: "wikidata/28205611",
    name: "RIPscrip version 1 Hot Icon",
    extensions: &["hot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
