use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850050: FileFormat = FileFormat {
    id: 105_850_050,
    puid: "wikidata/105850050",
    name: "Cabal info (with rem)",
    extensions: &["cabal"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
