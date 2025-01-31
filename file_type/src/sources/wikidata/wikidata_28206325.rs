use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206325: FileFormat = FileFormat {
    id: 28_206_325,
    puid: "wikidata/28206325",
    name: "Img Software Set Image Attributes",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
