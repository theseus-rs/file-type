use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50564741: FileFormat = FileFormat {
    id: 50_564_741,
    puid: "wikidata/50564741",
    name: "AVCHD Clip Information File",
    extensions: &["clpi", "cpi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
