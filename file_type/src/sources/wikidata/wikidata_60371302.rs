use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60371302: FileFormat = FileFormat {
    id: 60_371_302,
    puid: "wikidata/60371302",
    name: "Microsoft PowerPoint Macro-Enabled Show",
    extensions: &["ppsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
