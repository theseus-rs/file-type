use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_77051850: FileFormat = FileFormat {
    id: 77_051_850,
    puid: "wikidata/77051850",
    name: "Cal3D Xml Animation File",
    extensions: &["xaf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
