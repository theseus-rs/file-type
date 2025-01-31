use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205824: FileFormat = FileFormat {
    id: 28_205_824,
    puid: "wikidata/28205824",
    name: "CgBI",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
