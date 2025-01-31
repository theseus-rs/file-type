use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96147075: FileFormat = FileFormat {
    id: 96_147_075,
    puid: "wikidata/96147075",
    name: "Wolfram machine learning format",
    extensions: &["wmlf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
