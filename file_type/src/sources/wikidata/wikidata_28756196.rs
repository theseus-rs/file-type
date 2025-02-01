use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756196: FileFormat = FileFormat {
    id: 28_756_196,
    puid: "wikidata/28756196",
    name: "FWKCS NDX file",
    extensions: &["ndx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
