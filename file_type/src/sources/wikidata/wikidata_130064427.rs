use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130064427: FileFormat = FileFormat {
    id: 130_064_427,
    puid: "wikidata/130064427",
    name: "Koka file format",
    extensions: &["kk"],
    media_types: &["text/x-koka"],
    internal_signatures: &[],
    related_formats: &[],
};
