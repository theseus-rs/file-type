use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167850: FileFormat = FileFormat {
    id: 29_167_850,
    puid: "wikidata/29167850",
    name: "P-touch Editor Label",
    extensions: &["lbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
