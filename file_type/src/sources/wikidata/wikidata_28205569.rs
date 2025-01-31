use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205569: FileFormat = FileFormat {
    id: 28_205_569,
    puid: "wikidata/28205569",
    name: "Nokia Startup Logo",
    extensions: &["nsl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
