use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72271628: FileFormat = FileFormat {
    id: 72_271_628,
    puid: "wikidata/72271628",
    name: "ndr",
    extensions: &["ndr"],
    media_types: &["unknown"],
    internal_signatures: &[],
    related_formats: &[],
};
