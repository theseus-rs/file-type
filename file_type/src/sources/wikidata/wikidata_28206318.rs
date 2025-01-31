use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206318: FileFormat = FileFormat {
    id: 28_206_318,
    puid: "wikidata/28206318",
    name: "Img Software Set Colormapped Image",
    extensions: &["p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
