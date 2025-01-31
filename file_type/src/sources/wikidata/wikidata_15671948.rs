use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_15671948: FileFormat = FileFormat {
    id: 15_671_948,
    puid: "wikidata/15671948",
    name: "Blend file",
    extensions: &["blend", "blend"],
    media_types: &["application/x-blender", "application/x-blender"],
    internal_signatures: &[],
    related_formats: &[],
};
