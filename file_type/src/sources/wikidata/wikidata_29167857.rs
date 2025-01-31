use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167857: FileFormat = FileFormat {
    id: 29_167_857,
    puid: "wikidata/29167857",
    name: "P-touch Editor Lite Label",
    extensions: &["lbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
