use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121839915: FileFormat = FileFormat {
    id: 121_839_915,
    puid: "wikidata/121839915",
    name: "Encapsulated PostScript File Format 2.1",
    extensions: &["eps", "epsf"],
    media_types: &["application/postscript", "application/postscript"],
    internal_signatures: &[],
    related_formats: &[],
};
