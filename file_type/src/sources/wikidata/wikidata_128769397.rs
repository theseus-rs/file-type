use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128769397: FileFormat = FileFormat {
    id: 128_769_397,
    puid: "wikidata/128769397",
    name: "Concise Data Definition Language file",
    extensions: &["cddl"],
    media_types: &["text/x-cddl"],
    internal_signatures: &[],
    related_formats: &[],
};
