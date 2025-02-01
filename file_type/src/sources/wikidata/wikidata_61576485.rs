use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61576485: FileFormat = FileFormat {
    id: 61_576_485,
    puid: "wikidata/61576485",
    name: "Drawing Interchange File Format (ASCII), version R13",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
