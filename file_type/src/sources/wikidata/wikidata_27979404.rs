use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979404: FileFormat = FileFormat {
    id: 27_979_404,
    puid: "wikidata/27979404",
    name: "PICS",
    extensions: &["pcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
