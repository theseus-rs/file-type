use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4437542: FileFormat = FileFormat {
    id: 4_437_542,
    puid: "wikidata/4437542",
    name: "Direct Connect Hublist",
    extensions: &["dcls", "dclst", "xml.bz2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
