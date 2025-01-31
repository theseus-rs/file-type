use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50221292: FileFormat = FileFormat {
    id: 50_221_292,
    puid: "wikidata/50221292",
    name: "JEOL NMR Spectroscopy",
    extensions: &["jdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
