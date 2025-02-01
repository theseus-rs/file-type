use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59714459: FileFormat = FileFormat {
    id: 59_714_459,
    puid: "wikidata/59714459",
    name: "Encapsulated PostScript File Format",
    extensions: &["eps", "epsf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
