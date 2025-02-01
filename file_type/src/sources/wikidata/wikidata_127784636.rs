use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127784636: FileFormat = FileFormat {
    id: 127_784_636,
    puid: "wikidata/127784636",
    name: "Metal Shading Language File",
    extensions: &["metal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
