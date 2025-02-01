use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3841253: FileFormat = FileFormat {
    id: 3_841_253,
    puid: "wikidata/3841253",
    name: "MDL Molfile",
    extensions: &["mol"],
    media_types: &["chemical/x-mdl-molfile"],
    internal_signatures: &[],
    related_formats: &[],
};
